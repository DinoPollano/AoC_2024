mod data;

use sort::select_sort;
use std::collections::HashMap;
use crate::data::test_data;
use crate::data::load_data;

fn main() {
    let  (mut first, mut second) = load_data();
    // let (mut first, mut second ) = test_data();
    // select_sort(&mut first);
    // select_sort(&mut second);
    println!("Similarity: {}", similarity_score(&mut first, &mut second));
}

pub fn summed_difference(first: &mut Vec<i32>, second: &mut Vec<i32>) -> i32 {
    assert_eq!(first.len(), second.len());
    let mut total_difference: i32 = 0;
    for n in 0..first.len() {
        total_difference += (first[n] - second[n]).abs()
    }

     total_difference
}

pub fn similarity_score(first: &mut Vec<i32>, second: &mut Vec<i32>) -> i32{
    let mut repetitions = HashMap::new();
    let mut total_repetitions = 0;
    for f in &mut *first {
        repetitions.insert(*f, 0);
    }

    for s in second {
        let entry = repetitions.entry(*s).or_insert(0);
        *entry += 1;
    }

    for f in first{
        total_repetitions += *f * repetitions[f]
    }

    total_repetitions
}
