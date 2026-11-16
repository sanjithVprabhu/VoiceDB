# Week 01 — Foundations / Chapter 1

## Required reading

1. `curriculum/m00-foundations/01-ownership.md`
2. `curriculum/m00-foundations/03-exercises-ch1.md`
3. Start `curriculum/m00-foundations/00-theory.md` (systems; finish design note when ready)

## Coding

Primary work is in `crates/m00-core`. Get `cargo test -p m00-core` green first.

## Extra problems (Ch.1-tagged)

| Bin | Challenge |
| --- | --- |
| `run_length_encode` | Consume `Vec<u8>`, emit owned runs `(value, count)` |
| `dedup_sorted` | Consume sorted `Vec<i32>`, return unique owned vec |
| `flatten_owned` | Consume `Vec<Vec<T>>` into one `Vec<T>` without per-element clone |

Not a workspace member (kept under `problems/`). Run with:
`cargo test --manifest-path problems/week-01/Cargo.toml` (after you add tests)
or `cargo run --manifest-path problems/week-01/Cargo.toml --bin <name>`.

## Repeat queue

Anything > 40 minutes or needing a hint → `problems/repeat-queue.md`.
