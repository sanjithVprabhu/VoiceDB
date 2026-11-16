# Chapter 1 — Ownership, moves, and drop

**Track:** Rust R1  
**Crate:** `crates/m00-core`  
**Problems:** `problems/week-01`  
**Read time:** 45–60 minutes before coding  
**Goal:** build an intuitive feel for “who owns this memory, and when is it freed?”

Primary sources (read after this note, not instead of coding):

- [The Book — Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
- [The Book — References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html) (preview for Ch.2)
- [Rustonomicon — Working with unsafe / aliasing model](https://doc.rust-lang.org/nomicon/working-with-unsafe.html) (skim only; no unsafe this chapter)

---

## 1. What problem ownership solves

Most languages pick one of:

- **Manual free** (C): fast, easy to use-after-free / double-free / leak
- **GC** (Java, Go): safe, pays with pauses / hidden retention / poorer cache control
- **RC everywhere** (Swift ARC, `std::shared_ptr`): safe-ish, cycles and atomic traffic

Rust’s default is **unique ownership + move semantics**, with **borrowing** for temporary access. The compiler enforces *aliasing XOR mutability* for safe references. That is the whole game.

You want this intuition for VoiceDB later: a page buffer, a WAL record, or an audio chunk must have a clear owner so crash recovery and pooling do not guess.

---

## 2. Stack, heap, and values

- **Stack frames** hold locals whose size is known (or whose handle is known).
- **Heap** holds dynamically sized payloads (`String`, `Vec`, `Box`).
- A `String` on the stack is a fat handle: pointer + length + capacity. The bytes live on the heap.
- Moving a `String` moves the handle; the bytes are not copied. The old binding is invalidated.

`Copy` types (`u64`, `bool`, shared refs `&T`) duplicate the bit pattern freely. Everything else **moves** unless you `clone()`.

**Mental drill:** when you see `let b = a;` for a non-`Copy` value, ask: “did ownership transfer?” If yes, using `a` afterward must not compile.

---

## 3. Moves through functions

```text
fn take(s: String) { /* s dropped at end */ }
fn give() -> String { String::from("x") }

let s = give();   // you own it
take(s);          // moved in; s dead
// println!("{s}");  // error
```

Returning a value moves ownership out. Prefer returning owned data when the callee creates it; prefer borrowing when the caller retains ownership (Ch.2).

**Partial moves:** moving one field out of a struct can make the whole struct unusable unless you replace the field or only borrow.

---

## 4. Drop and RAII

When an owner goes out of scope, `Drop` runs. That is how files close, locks release, and `Vec` frees its buffer — without a `finally` block.

Order: fields drop in declaration order; reverse order for complex graphs is a later topic (`Drop` glue).

For storage engines, RAII is why a guard type around a mmap or file descriptor beats manual close calls scattered across error paths.

---

## 5. Clone vs Copy vs reuse

| Mechanism | Cost | When |
| --- | --- | --- |
| Move | ≈ memcpy of the handle | default transfer |
| `Copy` | bitwise duplicate | trivially copyable scalars |
| `Clone` | explicit deep (or smart) copy | you truly need two owners |
| Borrow (`&` / `&mut`) | no ownership change | temporary access (Ch.2) |

Cloning in a hot ingest path is a smell. Prefer moves and borrows; clone at API boundaries when you must retain a snapshot.

---

## 6. Ownership patterns you will reuse in VoiceDB

1. **Construct → move into store** — create a record, move it into a page / memtable (no shared mutable alias).
2. **Take → transform → return** — decode bytes into an owned struct, hand it back.
3. **Drain** — empty a `Vec` into a consumer without cloning elements (`drain`, `into_iter`).
4. **Replace** — `std::mem::replace` / `take` to move out of a `&mut` slot and leave a placeholder.

These show up in exercises below.

---

## 7. DSA lens for this chapter

Ownership-friendly algorithm shapes:

- **In-place mutation of an owned buffer** you fully control (`Vec<T>` / `[T; N]` after you own it)
- **Consume inputs, produce output** (merge two sorted `Vec`s into a new one)
- **Partition / reverse / rotate** without allocating a second full copy when not required

Borrowing-heavy DSA (two pointers on `&mut [T]`) is Chapter 2. Here you practice **consuming and producing owned collections** so move semantics become muscle memory.

---

## 8. Common compiler errors (decode them)

- *use of moved value* — ownership already transferred; redesign to borrow or clone deliberately
- *cannot move out of index* — indexed access into `Vec<String>` doesn’t give ownership; use `remove`, `swap_remove`, `drain`, or borrow
- *partial move* — put the field back, or borrow it, or restructure into `Option` + `take`

Do not silence these with `.clone()` until you can say why a clone is required.

---

## 9. Exit criteria for Chapter 1

You may advance when:

1. All `m00-core` Chapter 1 tests in `tests/spec.rs` pass (including adversarial cases)
2. You can explain, without notes, why `let b = a` invalidates `a` for `String` but not for `u32`
3. You re-implement `merge_sorted` and `rotate_right` from blank in < 30 minutes
4. Week-01 problems marked Ch.1 are cleared or on the repeat queue with a date

Then: Chapter 2 (borrowing) and continue the M0 design note in parallel.
