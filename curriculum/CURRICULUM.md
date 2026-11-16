# VoiceDB Curriculum — Rust dexterity × systems engine

Two tracks run in parallel. The **Rust track** builds coding intuition and DSA
fluency. The **Systems track** builds the voice telemetry database. Every Rust
concept is practiced until it feels automatic; then it is reused inside the
engine module that needs it.

**Exit rule (any unit):** tests green including adversarial cases; one benchmark
with a named bottleneck; a short design/notes paragraph; re-implement the core
from blank in under an hour; clear the problem set (or park hard items on the
repeat queue).

---

## How a unit works

1. Theory (Rust semantics + CS / systems prior art)
2. Worked mental models (you type examples; Tutor explains, does not implement)
3. Robust exercises in `crates/*` (`todo!()` → you fill)
4. DSA drills woven into the same concept
5. Week problems under `problems/week-NN`
6. Optional Reviewer pass on your code

---

## Phase map (24 weeks + bonus)

| Phase | Weeks | Systems module | Rust / CS spine |
| --- | --- | --- | --- |
| 0 Foundations | 1 | Data model decisions | Ownership, borrowing, lifetimes, slices, Result/Option, iterators |
| 1 Storage core | 2–4 | Pages, WAL, crash recovery | `unsafe` boundaries, alignment, endianness, RAII, interior mutability |
| 2 Ingest server | 5–6 | Framing, backpressure, TCP/Unix | async basics, channels, buffering, zero-copy views |
| 3 Indexes | 7–10 | B-Tree / LSM / inverted / time index | generics, trait objects, allocators, cache locality, DSA trees/heaps |
| 4 Query engine | 11–14 | Plans, scans, joins on time | enums-as-ASTs, visitors, SIMD-ish loops, complexity analysis |
| 5 Signal / extract | 15–17 | Audio chunks, features | `ndarray`-style thinking without hiding cost, FFI later |
| 6 Cryptography | 18–19 | At-rest / in-transit story | constant-time habits, newtypes, secrecy wrappers |
| 7 Distribution | 20–23 | Replication, consensus lite | atomics, memory ordering, locks, failure detectors |
| 8 Consolidation | 24 | Glue, demo path | API design, error taxonomy, observability |
| 9 Bonus | after | Multi-tenancy | quotas, isolation, authn/z surfaces |

---

## Rust track — exhaustive concept list

Order is deliberate: each block assumes the previous. DSA items in *italics*
are practiced in the same week as the language concept.

### R0 — Bootstrap fluency (assumed light; patch gaps only)
- Cargo workspace, `rustfmt`, `clippy`, `cargo test` / `nextest`, modules, visibility
- Scalar types, tuples, arrays, `String` vs `&str`, `Vec`
- `if` / `match` / `loop`, functions, methods
- *DSA:* frequency map on `&str`, two-sum on sorted slice (read-only)

### R1 — Ownership and moves  ← **Chapter 1 (this week’s coding start)**
- Stack vs heap; drop; move vs copy (`Copy` / `Clone`)
- Ownership transfer through functions; partial moves
- *DSA:* in-place reverse / rotate under exclusive ownership; merge two owned vecs

### R2 — Borrowing and references
- Shared vs mutable borrows; non-lexical lifetimes intuition
- Borrow checker as aliasing XOR mutability
- *DSA:* two-pointer / sliding window on `&[T]`; Dutch national flag with `&mut [T]`

### R3 — Slices, patterns, and APIs
- `&[T]`, `&mut [T]`, split/join, chunking
- Pattern matching depth; `ref` / `ref mut`; destructuring
- *DSA:* binary search; kadane; prefix sums — all slice-native

### R4 — Lifetimes
- Explicit lifetime parameters; structs holding references
- Lifetime elision rules; `'static` myths
- *DSA:* returning sub-slices; arena-style string interning sketch (API only)

### R5 — Option, Result, error design
- Combinators vs `?`; custom error enums; `thiserror` later
- Invariants expressed as types
- *DSA:* parse-and-validate pipelines; fallible graph edge lookup

### R6 — Structs, enums, algebraic data
- Newtypes; enum payloads; exhaustiveness
- *DSA:* expression AST eval; tagged union event model (feeds VoiceDB kinds)

### R7 — Traits and generics
- Trait bounds, orphan rules, blanket impls, associated types
- Monomorphisation vs `dyn Trait`
- *DSA:* generic heap / priority queue; trait-based graph traversal

### R8 — Iterators and closures
- `Iterator` / `IntoIterator`; adapter fusion; owning vs borrowing iterators
- FnOnce / FnMut / Fn
- *DSA:* streaming frequency; external merge-sort style chunk iterators

### R9 — Collections and memory intuition
- `HashMap` / `BTreeMap` / `VecDeque` / `BinaryHeap` — when and why
- Amortised growth; reallocation; capacity discipline
- *DSA:* LRU sketch; topological sort; interval merge (VoiceDB-adjacent)

### R10 — Smart pointers and interior mutability
- `Box`, `Rc`/`Arc`, `RefCell`/`Mutex`, `Cow`
- Cycle risks; API pressure that forces `Rc`
- *DSA:* adjacency list graphs; shared forest for interval indexes

### R11 — Concurrency foundations
- Threads; channels; `Send`/`Sync`; scoped threads
- Data races vs race conditions
- *DSA:* parallel map-reduce toy; work-stealing intuition (no crate required)

### R12 — Async readiness (used in ingest)
- Futures mental model; pinning intuition; executors as schedulers
- Cancellation and backpressure
- *DSA:* async bounded queue consumer patterns

### R13 — `unsafe` and FFI boundary (storage / signal)
- When unsafe is justified; soundness obligations
- Raw pointers, transmute discipline, MaybeUninit
- *DSA:* hand-rolled arena; endian-aware byte codecs

### R14 — Testing, property tests, benchmarks
- Unit / integration / adversarial / `proptest` later
- `criterion` / `hyperfine`; reading `perf` lightly
- Always: empty input, overflow, OOO, torn/crash scenarios where relevant

---

## Systems track — module detail

### M0 Foundations (week 1)
- Theory: time-aligned multi-track prior art (`00-theory.md`)
- Design note: nine decisions (`02-design-note.md`) — review before heavy code
- Rust: R1–R4 coding chapter + week-01 problems
- Deliverable: design note drafted; ownership exercise crate green

### M1 Storage core (weeks 2–4)
- Pages, checksums, freelist, WAL, replay, fsync discipline
- Rust: R9–R10, start R13; *DSA:* buffer pool clock/LRU, checksummed slabs

### M2 Ingest server (weeks 5–6)
- Length-prefixed framing, backpressure, connection lifecycle
- Rust: R11–R12; *DSA:* ring buffers, token bucket

### M3 Indexes (weeks 7–10)
- Time index, secondary indexes, (inverted for text labels)
- Rust: R7 deep dive; *DSA:* B-Tree ops, LSM compaction scheduling, interval trees

### M4 Query engine (weeks 11–14)
- Predicate pushdown, time-range scans, overlap joins
- Rust: R6–R8 as AST/plan; *DSA:* sort-merge, hash join, sweep-line

### M5 Signal / extraction (weeks 15–17)
- Chunked audio, feature windows, provenance of derived tracks
- Rust: numeric loops, R13 sample; *DSA:* convolution-style windows, reservoir sample

### M6 Cryptography (weeks 18–19)
- Key handling, AEAD at rest, TLS posture (design + minimal impl)
- Rust: newtypes, zeroize habits; no hand-rolled crypto primitives

### M7 Distribution (weeks 20–23)
- Replication log, leader election lite, conflict story for single-tenant first
- Rust: atomics, R11 advanced; *DSA:* quorum simulation, Lamport/vector clocks

### M8 Consolidation (week 24)
- End-to-end demo path, error taxonomy, README-level operator story
- Capstone checklist against PROGRESS.md exit criteria

### M9 Bonus — multi-tenancy
- Isolation, quotas, auth surfaces — only after week 24 exit

---

## Chapter → crate / problem mapping

| Chapter | Curriculum path | Code | Problems |
| --- | --- | --- | --- |
| M0 theory | `curriculum/m00-foundations/00-theory.md` | — | design note |
| Ch.1 Ownership | `curriculum/m00-foundations/01-ownership.md` | `crates/m00-core` | `problems/week-01` |
| Later Rust chapters | `curriculum/m00-foundations/01b-…` then per-module | matching crate | matching week |
| M1+ | `curriculum/m0N-*/` | `crates/m0N-*` | `problems/week-NN` |

---

## Parallel build plan (how we author material)

1. **Now:** Chapter 1 (ownership) fully scaffolded — you start coding.
2. **While you code Ch.1:** author fills remaining M0 Rust chapters (R2–R4) + week-01 extras.
3. **After Ch.1 green:** you draft design note; we open M1 scaffolding in parallel with your R5–R8 practice.
4. Never skip exit criteria to “save time” — flag if you’re about to.

---

## Primary sources (prefer these over summaries)

- [The Rustonomicon](https://doc.rust-lang.org/nomicon/) — aliasing, unsafe
- [Rust Reference — ownership / lifetimes](https://doc.rust-lang.org/reference/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Perfetto / Matroska / Arrow docs linked from M0 theory
- CLRS or Erickson for DSA proofs when an exercise asks “why O(…)”
