#!/usr/bin/env bash
# Bootstrap the curriculum repo. Run once, from inside the repo root:
#   bash bootstrap.sh
set -euo pipefail

echo "Scaffolding curriculum repo in $(pwd)"

# ---------------------------------------------------------------- folders ---
MODULES=(
  "m00-foundations"
  "m01-storage-core"
  "m02-ingest-server"
  "m03-indexes"
  "m04-query-engine"
  "m05-signal-extraction"
  "m06-cryptography"
  "m07-distribution"
  "m08-consolidation"
  "m09-multi-tenancy-bonus"
)

for m in "${MODULES[@]}"; do
  mkdir -p "curriculum/$m"
done

for w in $(seq -w 1 24); do
  mkdir -p "problems/week-$w/src/bin"
done

mkdir -p crates .cursor/rules .github/workflows

# ------------------------------------------------------- cargo workspace ---
cat > Cargo.toml <<'EOF'
[workspace]
resolver = "2"
members = []

[workspace.package]
edition = "2021"
rust-version = "1.80"

[workspace.dependencies]
# shared deps go here as modules are added
EOF

# ------------------------------------------------------- agent contract ----
cat > CLAUDE.md <<'EOF'
# Agent contract

This repository is a six-month self-taught systems course. The capstone is a
voice telemetry database built from scratch in Rust. The learning is the point;
finished code is only evidence of it.

## Scope

Single-tenant engine plus one demo UI. Multi-tenancy is a bonus chapter after
week 24. No business work: no design partners, pricing, or certification.

## Modes

Every request names a mode. If no mode is named, ask which one before acting.

### Tutor
MAY: explain concepts, compare designs, describe tradeoffs, quiz me, point at
papers and book chapters, review and argue with my design notes, explain why a
production system made a given choice.
MUST NOT: write implementation code for anything listed as a learning
objective, including as an illustrative example or pseudocode close enough to
transcribe.

### Reviewer
MAY: critique code I have already written, find bugs, name missing edge cases,
propose benchmarks, compare my approach against production engines, point at
the specific line where a problem lives.
MUST NOT: rewrite the code. Describe the problem and let me fix it. Do not post
a corrected version "for reference".

### Builder
MAY: write test harnesses, benchmark rigs, CLI plumbing, data generators, CI
config, the demo UI, and anything under xtask/.
MUST NOT: touch crates/*/src or problems/. Those are learning objectives.

## Course-author tasks

When asked to generate a module, produce theory, exercises, failing tests in
tests/spec.rs, type signatures with todo!() stubs, benchmark targets, and the
week's problem list. Never implement a stub. Tests must include adversarial
cases: crash mid-write, torn pages, out-of-order input, empty input, and
concurrent access where relevant.

## Standing rules

- Do not answer a question I have not asked. If my design note is wrong, say
  where and why; do not hand me the fix.
- Prefer pointing me at the primary source over summarising it.
- Flag when I am about to skip an exit criterion.
- Explain why a design exists and what it replaced, not just how to write it.
EOF

cp CLAUDE.md AGENTS.md

cat > .cursor/rules/course.mdc <<'EOF'
---
description: Course rules - agent acts as tutor, reviewer, or builder, never as implementer
alwaysApply: true
---

See CLAUDE.md in the repo root for the full contract. Summary:

- This repo is a systems course. I write all implementation code.
- Three modes: Tutor (explains, never writes implementation code), Reviewer
  (critiques my code, never rewrites it), Builder (harnesses, benchmarks, CLI,
  UI, xtask only).
- Never write or complete code under crates/*/src or problems/.
- When generating module material, write theory, exercises, failing tests, and
  todo!() stubs only.
- If no mode is named in my request, ask which mode before acting.
EOF

cat > .cursorignore <<'EOF'
# Keep my problem work out of the agent's context
problems/

# Build artifacts
target/
EOF

# --------------------------------------------------------------- tracking ---
cat > PROGRESS.md <<'EOF'
# Progress

**Scope (locked):** single-tenant voice telemetry engine + one demo UI.
Multi-tenancy is a bonus chapter after week 24. No business work.

**Rule:** a module is done only when all five hold - tests green including
adversarial cases, a benchmark recorded with one bottleneck fixed, a design
note written, the core structure re-implementable from blank in under an hour,
and the module's problem set cleared.

## Modules

- [ ] 0. Foundations (week 1)
- [ ] 1. Storage core (weeks 2-4)
- [ ] 2. Ingest server (weeks 5-6)
- [ ] 3. Indexes (weeks 7-10)
- [ ] 4. Query engine (weeks 11-14)
- [ ] 5. Signal and extraction (weeks 15-17)
- [ ] 6. Cryptography (weeks 18-19)
- [ ] 7. Distribution (weeks 20-23)
- [ ] 8. Consolidation (week 24)
- [ ] 9. Multi-tenancy (bonus)

## Daily log

<!-- one line per day: built / broke / did not understand -->

## Weekly metrics

| Week | Build hrs | Problem hrs | Problems | Median time | Repeat queue | Note |
| --- | --- | --- | --- | --- | --- | --- |
EOF

cat > problems/repeat-queue.md <<'EOF'
# Repeat queue

Anything that took over 40 minutes or needed a hint. Re-solve at +1 week,
+1 month, +3 months.

| Problem | Topic | First solved | Time | Due next | Attempts |
| --- | --- | --- | --- | --- | --- |
EOF

# ---------------------------------------------------------------- ci --------
cat > .github/workflows/ci.yml <<'EOF'
name: ci
on:
  push:
  pull_request:

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy, rustfmt
      - uses: Swatinem/rust-cache@v2
      - run: cargo fmt --all -- --check
      - run: cargo clippy --all-targets -- -D warnings
      - run: cargo test --all
EOF

cat > .gitignore <<'EOF'
target/
*.wav
*.opus
data/
EOF

echo "Done. Next: git init && git add -A && git commit -m 'scaffold'"