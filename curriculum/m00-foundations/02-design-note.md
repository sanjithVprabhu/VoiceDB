# Module 0 — Design note (nine decisions)

Draft **after** reading `00-theory.md`. Target three to five pages.

For each decision: **choice**, **alternatives rejected**, **why**. If unsure, write what evidence would settle it — do not guess.

Get this reviewed (**Reviewer** mode) before heavy storage code. Cheap now; expensive in week 11.

---

## 1. Identity

Format and generator for conversation, track, and event ids. Client or server generated? One id or an external/internal pair?

**Your answer:**

## 2. Time representation

Unit, width, and origin. Fixed ticks or a per-track timescale? What range and precision do you actually need?

**Your answer:**

## 3. The master clock rule

Which stream defines zero. What happens to events before any audio exists. What you store so alignment can be undone later.

**Your answer:**

## 4. Track kinds

Are Audio / Point / Interval / Signal sufficient? Test against: stereo audio, a word with confidence, a 512-d embedding every 250 ms, an ongoing “on hold” state, and a call-level label. Where do vector-valued signals live?

**Your answer:**

## 5. Attributes

Typed per-kind structs, a generic map, or promoted subcolumns. Are keys interned? What is the closed set of value types?

**Your answer:**

## 6. Event vocabulary

Every kind — `word`, `turn`, `barge_in`, `stt`, `llm`, `tts`, `tool_call`, `silence`, `dtmf`, `transfer`, plus any you add — marked point or interval, with fields and types.

**Your answer:**

## 7. Derivation

How a track records its producer, how staleness is detected or made impossible, and what happens to derived data when a parent is deleted.

**Your answer:**

## 8. Boundaries

Unterminated intervals at call end. Zero-length intervals. Overlapping words from two STT engines. Timestamps before zero. Events arriving out of order. Duplicate deliveries.

**Your answer:**

## 9. Invariants

Statements that must always hold, written so a property test could check each one. (“Intervals within a track never overlap” is testable; “timestamps are correct” is not.)

**Your answer:**
