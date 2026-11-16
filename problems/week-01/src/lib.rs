//! Week 01 extra ownership problems. Stubs only — you implement.

/// Run-length encode a byte vector into owned `(byte, count)` pairs.
/// Empty input → empty output. Counts are `usize` and must be > 0.
pub fn run_length_encode(input: Vec<u8>) -> Vec<(u8, usize)> {
    let _ = input;
    todo!()
}

/// `input` is sorted ascending. Return unique values, consuming `input`.
pub fn dedup_sorted(input: Vec<i32>) -> Vec<i32> {
    let _ = input;
    todo!()
}

/// Flatten nested owned vectors into one vector (move elements, do not clone `T`).
pub fn flatten_owned<T>(nested: Vec<Vec<T>>) -> Vec<T> {
    let _ = nested;
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn stub_rle_panics_until_implemented() {
        let _ = run_length_encode(vec![1, 1, 2]);
    }
}
