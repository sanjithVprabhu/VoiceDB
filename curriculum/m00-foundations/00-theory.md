# Module 0 — Theory: modelling time-aligned multi-track data

> Save path: `curriculum/m00-foundations/00-theory.md`  
> Read time: about 90 minutes.  
> Purpose: design the data model the next 23 weeks are built on. These notes summarise prior art and costs. They deliberately do **not** tell you what to choose.

Work through sections 1–8, then answer the nine decisions in `02-design-note.md`.

---

0. The problem you are modelling
A voice conversation is several parallel streams of different shapes that must be
interpretable against one shared timeline:
Continuous samples (audio, one stream per speaker)
Labelled spans (words, turns, pipeline latencies, tool calls)
Instants (barge-in, DTMF press, hangup)
Sampled numeric or vector values (volume, sentiment, embeddings)
Facts about the whole conversation (outcome, agent version)
Anything you can store, you must also be able to ask temporal questions about: what
overlapped what, what followed what within N milliseconds, what was happening at time T.
Four families of prior art solved parts of this. None solved all of it, which is why the design
is interesting.

1. Media containers: Matroska and MP4
What they are. Container formats that interleave audio, video, subtitle and chapter tracks
in one file, so a player can seek to any point and get all tracks in sync.
What to take from them.
Timescale per track. MP4 does not store seconds. Each track declares a timescale (ticks per

second) in its media header, and all timestamps in that track are integer ticks. Audio tracks
usually set the timescale to the sample rate, so a timestamp is a sample index. The movie
header carries a separate global timescale. This solves rounding: a 44,100 Hz track
expressed in milliseconds accumulates error, in ticks it does not.



The cost. Converting between tracks means integer rescaling everywhere, and rescaling

mismatched timescales loses precision. Every MP4 library has bugs here.

Cluster/segment layout. Matroska groups all tracks' data into clusters covering a short time

range, each with an absolute timestamp, and stores deltas within the cluster. Seeking
means finding a cluster, not scanning. This is the same idea you will need for audio chunking
and for time-partitioned event storage.
Edit lists and gaps. MP4 has an "edit list" that maps presentation time onto media time,

because real recordings start late, pause, and have gaps. Ask yourself where your
equivalent lives: when the caller's audio starts 4 seconds before the agent joins, something
has to represent that offset.
Read: Matroska element spec ( matroska.org/technical/elements.html ) — Timestamps,
Clusters, TrackEntry. For MP4, search "ISOBMFF mvhd timescale mdhd" and read a
container-format walkthrough on wiki.multimedia.cx .

2. Trace formats: Perfetto, Chrome Trace Event,
OpenTelemetry
What they are. Formats for recording what a system did, as events on a timeline, produced
by many sources at once.
What to take from them.
The three event shapes. Chrome Trace Event Format settled on: duration events (begin/end

pairs, forming nested slices), instant events, and counter events. Perfetto keeps that shape
in a binary protobuf form. This is almost exactly Interval / Point / Signal. That two
independent systems converged here is evidence the decomposition is natural — and a
reason to check what they could not express.
Interning. Perfetto does not repeat strings. Names, categories and attribute keys are

interned into a table and referenced by integer id, per trace. At millions of events, the string
table is the difference between a usable format and a bloated one. Your word events will
repeat a few thousand distinct strings across millions of instances.
Packet sequences and clock snapshots. Perfetto tags each packet with its producing
sequence and periodically emits clock snapshots — the same instant read on several clocks

(monotonic, boottime, realtime) — so a reader can convert between them after the fact
rather than forcing producers to agree at write time. Consider this against your masterclock rule.
OpenTelemetry's attributes. Spans carry a flat map of typed attributes (string, bool, int,

double, arrays). Resource attributes describe the producing entity and are stored once, not
per span. OTel also distinguishes parent (causal nesting) from links (non-hierarchical



relationships). Note the cost: the attribute map is flexible but the format spends bytes on
keys, and semantic conventions exist precisely because free-form keys become chaos.
Read: perfetto.dev/docs/reference/trace-packet-proto , the Chrome Trace Event Format
doc, opentelemetry.io/docs/specs/otel/trace/api .

3. Columnar type systems: Arrow and Parquet
What they are. In-memory (Arrow) and on-disk (Parquet) representations for analytical
data, designed for scanning millions of rows fast.
What to take from them.
FixedSizeList. A 512-dimension embedding is FixedSizeList<Float32, 512> : one contiguous

buffer, no per-row offsets, and SIMD-friendly. If your Signal kind holds vectors, this is what
it compiles down to. Consider also FixedSizeBinary for quantised (int8 or binary)
embeddings.
Dictionary encoding. Arrow's answer to Perfetto's interning: a column of repeated strings

becomes indices into a dictionary. Cheap equality, cheap filtering, small files.

Struct vs Map. Arrow distinguishes Struct (fixed named fields, each its own column, nulls

cheap) from Map (arbitrary key-value pairs per row). Struct scans fast and validates at
write time; Map absorbs new fields without migration. This is decision 5 in concrete form.

Schema evolution in Parquet. Fields are matched by name and id, new nullable columns can

be added, and readers tolerate missing columns. What it will not do cheaply is change a
column's type or unnest a Map into columns retroactively. Whatever you choose, you
inherit its migration story.

Read: arrow.apache.org/docs/format/Columnar.html — the sections on physical memory
layout, dictionary encoding, and nested types. Then skim parquet.apache.org/docs/fileformat/ on schema and metadata.

4. Speech annotation: Praat TextGrid and ELAN EAF
What they are. Formats phoneticians and linguists have used for decades to annotate
recordings: the closest existing thing to what you are building.
What to take from them.
Tiers. A TextGrid is a set of tiers, each either an interval tier (a contiguous partition of the
timeline into labelled intervals, including empty ones) or a point tier (labelled instants).



Tiers are independent: word tier, phoneme tier, speaker tier, noise tier. Your "tracks" are
tiers.
Intervals tile the timeline. In a TextGrid, interval tiers have no gaps: silence is an interval with

an empty label. This has a real consequence — "what was happening at time T on this tier"
is always answerable with one lookup, and there is no ambiguity between "nothing
recorded" and "nothing happened".

ELAN's hierarchy. EAF adds dependent tiers: a translation tier whose annotations are bound

to a parent tier's time slots, so children cannot drift out of alignment with their parent.
Compare this with your derivation and staleness problem — ELAN encodes the
dependency structurally, in the format, rather than as metadata.

Time slots. EAF stores timestamps once in a time-slot table and annotations reference slot

ids, so two annotations sharing a boundary share the value exactly. Boundary equality
becomes identity rather than float comparison.

Read: Praat manual, "TextGrid file formats" ( fon.hum.uva.nl/praat/manual ). ELAN EAF
documentation at archive.mpi.nl/tla/elan/documentation .

5. Identity: ULID, UUIDv7, snowflake ids, monotonic
counters
The axes that matter.
Who generates it. Client-generated ids let an SDK buffer offline and retry without

coordination, and make writes idempotent. Server-generated ids let you guarantee
uniqueness and hide creation rate.
Sortability. Time-prefixed ids (ULID, UUIDv7, snowflake) cluster temporally adjacent
records together on disk, which is exactly what you want for time-partitioned storage and
terrible for hot-partition spread in a sharded write path.
Width. 128-bit ids cost 16 bytes everywhere they appear — every event row that references
a conversation. A per-tenant u64, or an internal dense integer mapped from the external id,
costs 8 or fewer.
Leakage. A time-prefixed id in a URL tells anyone when the record was created, and
snowflake-style ids can leak volume.
The two-id pattern. Many systems carry an external id (stable, opaque, client-visible) and an

internal dense id (small, sequential, used in storage and joins). It costs a mapping table and
buys narrow columns.
Read: github.com/ulid/spec , RFC 9562 (UUIDv6/7/8) at
datatracker.ietf.org/doc/html/rfc9562 .



6. Clocks: drift, skew, and why two audio streams
disagree
This is the hardest part of decision 3, and it is a physical problem, not a software one.
Sample clocks drift. Two devices sampling "16,000 Hz" use separate crystal oscillators,

typically accurate to tens of parts per million. At 50 ppm, two streams diverge by about 3
ms per minute — enough to matter over a 20-minute call when you are measuring 300 ms
interruptions.
Networks add jitter and loss. Over RTP, packets arrive late, out of order, or not at all. A

receiver's jitter buffer (WebRTC's NetEq is the reference implementation) hides this by
stretching and compressing audio, which means the played timeline is not the captured
timeline. Concealed packet loss creates audio that never existed at a timestamp that looks
legitimate.
Wall clocks jump. NTP steps, leap-second smearing, suspend/resume. Any timestamp

derived from realtime can move backwards. Monotonic clocks do not, but they are only
comparable within one process's lifetime, which is why Perfetto emits clock snapshots.
Questions this forces. Which stream is authoritative? What do you record so that a future

reader can undo your alignment decision? Do you store both the raw producer timestamp
and the aligned one? Is drift corrected at ingest (resampling, timestamp rewriting) or
recorded as metadata and corrected at read time?
Read: search "RTP timestamp vs NTP timestamp RTCP sender report" for how RTP itself
solves cross-stream sync, and "WebRTC NetEq jitter buffer" for what happens to the
timeline in practice.

7. Attributes: typed columns, maps, or both
The three shapes, and what each costs:



Shape
Typed struct per
event kind

Write
Fast,
validated

Scan

Evolution

Errors
caught

Fastest: only touched
columns read

Migration per new
field

At write
time

Generic key-value Fast,
Slower: keys stored per
Free
map
unvalidated row, no column pruning
Map with promoted
Fast
subcolumns

Fast for promoted keys

Never

Free, promotion is a
Partially
background job

The third row is what production telemetry systems converge on. ClickHouse materialises
frequently-used keys out of a Map or JSON column into real subcolumns; several vendors
auto-promote hot keys. The cost is complexity and two code paths for reads.
Worth deciding separately: whether attribute keys are interned (section 2), and whether
values are a closed set of types or arbitrary.
Read: clickhouse.com/docs/en/sql-reference/data-types/map and the JSON data type page
beside it.

8. Provenance and staleness
You will re-run a better ASR model over old audio. Then a better one. Then the same model
with a different beam width. A year later you must be able to answer "what produced this
transcript, and is it still current?"
What identifies a producer. Name and version are the obvious fields and are not sufficient:

weights change without version bumps, and configuration (language hint, beam size, VAD
threshold, prompt) changes output. Hosted APIs change silently. A content hash over the
full configuration — plus, if you can get it, a model identifier the vendor commits to — is the
stronger answer, and costs you readability.
Lineage. Two shapes exist. Pointer-based: each derived track names its parent, and

staleness is computed by walking up. Content-addressed: a derived track's identity
includes the hash of its inputs and its producer config, so "stale" is not a state at all — a
changed input simply produces a different identity, and the old one remains valid for the
old input. This is how build systems (Bazel, Nix) avoid staleness entirely, and it is worth
understanding before you pick the pointer approach by default.
Deletion interacts with this. If a caller asks for their audio to be deleted, what happens to the

transcript derived from it, and to the embedding derived from that? Decide whether
derived data inherits the deletion.



Read: skim the Nix or Bazel documentation on content-addressed derivations; "data
lineage" in the Apache Atlas or OpenLineage docs for the pointer style.

9. Suggested reading order
If you read only three things, read these, 20 minutes each:
1. Perfetto trace packet reference — event shapes, interning, clock snapshots
2. Matroska TrackEntry and Timestamps — tracks, timescales, clusters
3. Arrow columnar format, nested types section — FixedSizeList, Dictionary, Struct vs Map
Then Praat TextGrid (10 minutes) because it is the closest analogue to what you are
building, and the clock material in section 6 if decision 3 stalls you.

10. The nine decisions
Answer each in 02-design-note.md : the decision, the alternatives you rejected, and why.
Where you are unsure, write down what evidence would settle it rather than guessing.
1. Identity. Format and generator for conversation, track and event ids. Client or server
generated? One id or an external/internal pair?
2. Time representation. Unit, width, and origin. Fixed ticks or a per-track timescale? What
range and precision do you actually need?
3. The master clock rule. Which stream defines zero. What happens to events before any
audio exists. What you store so alignment can be undone later.
4. Track kinds. Are Audio / Point / Interval / Signal sufficient? Test them against: stereo audio,
a word with confidence, a 512-d embedding every 250 ms, an ongoing "on hold" state, and a
call-level label. Where do vector-valued signals live?
5. Attributes. Typed per-kind structs, a generic map, or promoted subcolumns. Are keys
interned? What is the closed set of value types?
6. Event vocabulary. Every kind — word , turn , barge_in , stt , llm , tts , tool_call ,
silence , dtmf , transfer , plus any you add — marked point or interval, with its fields and
their types.
7. Derivation. How a track records its producer, how staleness is detected or made
impossible, and what happens to derived data when a parent is deleted.
8. Boundaries. Unterminated intervals at call end. Zero-length intervals. Overlapping words
from two STT engines. Timestamps before zero. Events arriving out of order. Duplicate
deliveries.
9. Invariants. The statements that must always hold, written so a property test could check
each one. "Intervals within a track never overlap" is testable; "timestamps are correct" is
not.



Target three to five pages. When it is drafted, get it reviewed before writing any code — the
review is cheap now and expensive in week 11.



