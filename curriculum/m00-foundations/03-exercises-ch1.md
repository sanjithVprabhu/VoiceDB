# Chapter 1 — Exercises (Ownership)

Implement the stubs in `crates/m00-core`. Run:

```bash
cargo test -p m00-core
# or
cargo nextest run -p m00-core
```

Do **not** use `.clone()` unless a test or docstring explicitly allows it. Prefer moves, `drain`, `into_iter`, `mem::take`, and rebuilding outputs.

---

## API checklist

| Fn | Idea | DSA / systems link |
| --- | --- | --- |
| `consume_and_len` | move a `String` in; return length | moves through functions |
| `swap_strings` | exchange two owned strings | `mem::swap` / destructuring |
| `drain_to_vec` | drain all from `&mut Vec<T>` into a new `Vec` | buffer take for flush |
| `reverse_owned` | reverse an owned `Vec` in place; return it | in-place array algo |
| `rotate_right` | rotate owned `Vec` right by `k` | classic rotate |
| `merge_sorted` | merge two sorted `Vec<i32>` into one sorted | merge step of mergesort |
| `take_field` | move one field out via `Option::take` | partial move pattern |
| `partition_owned` | stable-ish split into (lt, ge) by pivot | quicksort partition flavour |
| `into_chunks` | consume `Vec<T>`, yield owned chunks of size `n` | batching for ingest |

Adversarial cases covered in `tests/spec.rs`: empty inputs, `k == 0`, `k > len`, unequal merge lengths, chunk size 0 / oversized, already-sorted / reverse-sorted merges.

---

## Suggested order

1. `consume_and_len`, `swap_strings`
2. `reverse_owned`, `rotate_right`
3. `drain_to_vec`, `take_field`
4. `merge_sorted`, `partition_owned`
5. `into_chunks`

---

## After green

1. Re-implement `merge_sorted` and `rotate_right` from blank (no peeking).
2. Skim week-01 problems; solve the Ch.1-tagged ones.
3. Start reading `00-theory.md` if you have not; fill design-note stubs when ready.
4. Ping **Tutor** for a quiz, or **Reviewer** with your impl.
