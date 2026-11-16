//! Module 0 / Chapter 1 — Ownership exercises.
//!
//! Implement the functions below. Prefer moves over clones.
//! See `curriculum/m00-foundations/01-ownership.md` and `03-exercises-ch1.md`.

/// Takes ownership of `s` and returns its length in bytes.
pub fn consume_and_len(s: String) -> usize {
    let _ = s;
    todo!()
}

/// Swaps the two strings in place (no clone).
pub fn swap_strings(a: &mut String, b: &mut String) {
    let _ = (a, b);
    todo!()
}

/// Drains all elements from `src` into a new `Vec`, leaving `src` empty.
pub fn drain_to_vec<T>(src: &mut Vec<T>) -> Vec<T> {
    let _ = src;
    todo!()
}

/// Reverses `items` in place and returns the same vector (ownership preserved).
pub fn reverse_owned<T>(mut items: Vec<T>) -> Vec<T> {
    let _ = &mut items;
    todo!()
}

/// Rotates `items` to the right by `k` positions.
///
/// `k` may be larger than `items.len()`; treat it modulo length when length > 0.
/// Rotating an empty vector is a no-op. `k == 0` is a no-op.
pub fn rotate_right<T>(mut items: Vec<T>, k: usize) -> Vec<T> {
    let _ = (&mut items, k);
    todo!()
}

/// Merges two sorted vectors (ascending) into one sorted vector.
///
/// Inputs are moved in and must not be cloned elementwise unless needed for the
/// algorithm's own temporaries. Equal values are allowed; stability is not required.
pub fn merge_sorted(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    let _ = (left, right);
    todo!()
}

/// A tiny record used to practice moving a field out without cloning.
#[derive(Debug, PartialEq, Eq)]
pub struct NamedBuffer {
    pub name: Option<String>,
    pub bytes: Vec<u8>,
}

/// Moves `name` out of `buf`, leaving `None` behind. Returns the taken name.
pub fn take_field(buf: &mut NamedBuffer) -> Option<String> {
    let _ = buf;
    todo!()
}

/// Partitions owned values around `pivot`: all `< pivot` then all `>= pivot`.
///
/// Relative order within each side is unspecified. Consumes `items`.
pub fn partition_owned(items: Vec<i32>, pivot: i32) -> Vec<i32> {
    let _ = (items, pivot);
    todo!()
}

/// Consumes `items` and splits into owned chunks of length `n`.
///
/// - If `n == 0`, return an empty `Vec` of chunks (no panic).
/// - The final chunk may be shorter than `n`.
/// - If `items` is empty and `n > 0`, return an empty list of chunks.
pub fn into_chunks<T>(items: Vec<T>, n: usize) -> Vec<Vec<T>> {
    let _ = (items, n);
    todo!()
}
