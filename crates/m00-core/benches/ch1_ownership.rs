//! Benchmark targets for Chapter 1.
//! Run once you have implementations:
//!   cargo bench -p m00-core --bench ch1_ownership
//!
//! Record one bottleneck in PROGRESS.md after you fix it.

use std::hint::black_box;
use std::time::Instant;

fn main() {
    // Harness without criterion dependency yet — wall-clock smoke targets.
    let n = 50_000usize;
    let left: Vec<i32> = (0..n as i32).map(|x| x * 2).collect();
    let right: Vec<i32> = (0..n as i32).map(|x| x * 2 + 1).collect();

    let t0 = Instant::now();
    let merged = m00_core::merge_sorted(black_box(left), black_box(right));
    let merge_ms = t0.elapsed().as_secs_f64() * 1e3;
    assert_eq!(merged.len(), n * 2);

    let items: Vec<i32> = (0..n as i32).collect();
    let t1 = Instant::now();
    let rotated = m00_core::rotate_right(black_box(items), black_box(n / 3));
    let rotate_ms = t1.elapsed().as_secs_f64() * 1e3;
    assert_eq!(rotated.len(), n);

    println!("merge_sorted n={n}: {merge_ms:.3} ms");
    println!("rotate_right n={n}: {rotate_ms:.3} ms");
    println!("target: understand which one allocates more; fix one bottleneck after profiling.");
}
