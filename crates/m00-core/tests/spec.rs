//! Adversarial + functional spec for Chapter 1 ownership exercises.
use m00_core::{
    consume_and_len, drain_to_vec, into_chunks, merge_sorted, partition_owned, reverse_owned,
    rotate_right, swap_strings, take_field, NamedBuffer,
};

#[test]
fn consume_and_len_empty_and_nonempty() {
    assert_eq!(consume_and_len(String::new()), 0);
    assert_eq!(consume_and_len(String::from("voice")), 5);
    assert_eq!(consume_and_len(String::from("🦀")), 4); // utf-8 byte length
}

#[test]
fn swap_strings_exchanges_without_losing_data() {
    let mut a = String::from("left");
    let mut b = String::from("right-hand");
    swap_strings(&mut a, &mut b);
    assert_eq!(a, "right-hand");
    assert_eq!(b, "left");
}

#[test]
fn drain_to_vec_empties_source() {
    let mut src = vec![1, 2, 3];
    let out = drain_to_vec(&mut src);
    assert!(src.is_empty());
    assert_eq!(out, vec![1, 2, 3]);

    let mut empty: Vec<i32> = vec![];
    assert!(drain_to_vec(&mut empty).is_empty());
    assert!(empty.is_empty());
}

#[test]
fn reverse_owned_empty_single_and_many() {
    let empty: Vec<i32> = vec![];
    assert!(reverse_owned(empty).is_empty());
    assert_eq!(reverse_owned(vec![7]), vec![7]);
    assert_eq!(reverse_owned(vec![1, 2, 3, 4]), vec![4, 3, 2, 1]);
}

#[test]
fn rotate_right_adversarial_k() {
    assert!(rotate_right(Vec::<i32>::new(), 5).is_empty());
    assert_eq!(rotate_right(vec![1, 2, 3], 0), vec![1, 2, 3]);
    assert_eq!(rotate_right(vec![1, 2, 3], 1), vec![3, 1, 2]);
    assert_eq!(rotate_right(vec![1, 2, 3], 3), vec![1, 2, 3]);
    assert_eq!(rotate_right(vec![1, 2, 3], 4), vec![3, 1, 2]); // 4 % 3 == 1
    assert_eq!(rotate_right(vec![1, 2, 3, 4, 5], 2), vec![4, 5, 1, 2, 3]);
}

#[test]
fn merge_sorted_empty_and_duplicates() {
    assert!(merge_sorted(vec![], vec![]).is_empty());
    assert_eq!(merge_sorted(vec![1, 3], vec![]), vec![1, 3]);
    assert_eq!(merge_sorted(vec![], vec![2, 4]), vec![2, 4]);
    assert_eq!(merge_sorted(vec![1, 3, 5], vec![2, 4, 6]), vec![1, 2, 3, 4, 5, 6]);
    assert_eq!(merge_sorted(vec![1, 1, 2], vec![1, 3]), vec![1, 1, 1, 2, 3]);
    assert_eq!(merge_sorted(vec![5, 6], vec![1, 2, 3, 4]), vec![1, 2, 3, 4, 5, 6]);
}

#[test]
fn take_field_twice_is_none() {
    let mut buf = NamedBuffer {
        name: Some(String::from("track-a")),
        bytes: vec![9, 9],
    };
    assert_eq!(take_field(&mut buf), Some(String::from("track-a")));
    assert_eq!(buf.name, None);
    assert_eq!(buf.bytes, vec![9, 9]);
    assert_eq!(take_field(&mut buf), None);
}

#[test]
fn partition_owned_empty_and_extremes() {
    assert!(partition_owned(vec![], 0).is_empty());

    let out = partition_owned(vec![3, 1, 4, 1, 5, 9, 2], 4);
    let split = out.iter().position(|&x| x >= 4).unwrap_or(out.len());
    assert!(out[..split].iter().all(|&x| x < 4));
    assert!(out[split..].iter().all(|&x| x >= 4));
    let mut sorted = out.clone();
    sorted.sort();
    assert_eq!(sorted, vec![1, 1, 2, 3, 4, 5, 9]);

    let all_lt = partition_owned(vec![1, 2, 3], 99);
    assert!(all_lt.iter().all(|&x| x < 99));
    let all_ge = partition_owned(vec![5, 6, 7], 0);
    assert!(all_ge.iter().all(|&x| x >= 0));
}

#[test]
fn into_chunks_adversarial() {
    assert!(into_chunks(Vec::<i32>::new(), 3).is_empty());
    assert!(into_chunks(vec![1, 2, 3], 0).is_empty());
    assert_eq!(
        into_chunks(vec![1, 2, 3, 4, 5], 2),
        vec![vec![1, 2], vec![3, 4], vec![5]]
    );
    assert_eq!(into_chunks(vec![1, 2, 3], 10), vec![vec![1, 2, 3]]);
    assert_eq!(into_chunks(vec![7], 1), vec![vec![7]]);
}
