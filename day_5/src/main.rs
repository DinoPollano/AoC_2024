mod data;
use std::{
    collections::{HashMap, HashSet},
    usize,
};

pub fn sort_pages(order_vec: Vec<(u32, u32)>) -> HashMap<u32, HashSet<u32>> {
    let mut order_map: HashMap<u32, HashSet<u32>> = HashMap::new();

    for (antecedant, subsequent) in order_vec {
        order_map.entry(antecedant).or_default().insert(subsequent);
        order_map.entry(subsequent).or_default();
    }
    for set in &order_map {
        println!("{} : {:?}", set.0, set.1);
    }
    order_map
}

pub fn check_pages_are_correct(
    sorted_map: &HashMap<u32, HashSet<u32>>,
    updates: &Vec<u32>,
) -> bool {
    let mut page = updates.iter().peekable();
    let mut correct: bool = true;
    while correct {
        let page_number = match page.peek() {
            Some(&value) => value,
            None => break,
        };
        if page.next() == None {
            break;
        }
        let order = match sorted_map.get(&page_number) {
            Some(set) => set,
            None => break,
        };

        let tail: HashSet<u32> = page.clone().copied().collect();
        correct = order.is_superset(&tail);
    }
    correct
}

pub fn sort_updates(sorted_map: &HashMap<u32, HashSet<u32>>, updates: &Vec<u32>) -> Vec<u32> {
    let mut page_tails: Vec<(usize, u32)> = updates
        .clone()
        .into_iter()
        .map(|x| {
            let length = match sorted_map.get(&x) {
                Some(set) => set.len(),
                None => 0,
            };
            (length, x)
        })
        .collect();
    page_tails.sort_by(|a, b| (a.0.cmp(&b.0).reverse()));
    page_tails.iter().map(|x| x.1).collect()
}

pub fn sort_updates2(sorted_map: &HashMap<u32, HashSet<u32>>, updates: &Vec<u32>) -> Vec<u32> {
    let mut sorted: Vec<u32> = updates.clone();

    let sorted_end_index = sorted.len() - 2;
    let mut n = sorted_end_index;
    println!("{:?}", updates);
    loop {
        let order = sorted_map.get(&sorted.get(n).unwrap()).unwrap();
        let outlier = sorted.iter().skip(n + 1).position(|x| !order.contains(x));
        match outlier {
            Some(outlier) => {
                let outlier_pos = n + 1 + outlier;
                let value = sorted.remove(outlier_pos);
                sorted.insert(n, value);
                n = sorted_end_index;
            }
            None => {
                if n > 0 {
                    n -= 1
                } else {
                    break;
                }
            }
        }
    }
    sorted
}

use data::{load_actual_data, load_test_data};
fn main() {
    println!("DAY 5");
    let (order_vec, updates_vec) = load_actual_data();
    let sorted_order = sort_pages(order_vec);
    let mut result: u32 = 0;
    let mut result_of_sorted: u32 = 0;
    for updates in updates_vec {
        if check_pages_are_correct(&sorted_order, &updates) {
            let middle_page_num = updates.len() / 2;
            result += updates.get(middle_page_num).unwrap();
        } else {
            let sorted_update = sort_updates2(&sorted_order, &updates);
            let middle_page_num = (sorted_update.len()) / 2;
            result_of_sorted += sorted_update.get(middle_page_num).unwrap();
        }
    }

    println!("unsorted: {}", result);
    println!("sorted: {}", result_of_sorted);
    println!("summed: {}", (result + result_of_sorted))
}

#[cfg(test)]
mod tests {
    use crate::{check_pages_are_correct, data::load_test_data, sort_pages, sort_updates2};
    #[test]
    fn test_check_pages_are_correct() {
        let (order_vec, updates_vec) = load_test_data();
        let sorted = sort_pages(order_vec);
        for (i, expected_result) in vec![true, true, true, false, false, false]
            .iter()
            .enumerate()
        {
            assert_eq!(
                check_pages_are_correct(&sorted, &updates_vec[i]),
                *expected_result,
                "{:?}",
                &updates_vec[i]
            );
        }
    }
    #[test]
    fn test_sort_updates() {
        let (order_vec, _) = load_test_data();
        let sorted = sort_pages(order_vec);
        assert_eq!(
            sort_updates2(&sorted, &(vec![75, 97, 47, 61, 53])),
            vec![97, 75, 47, 61, 53]
        );
        assert_eq!(
            sort_updates2(&sorted, &(vec![61, 13, 29])),
            vec![61, 29, 13]
        );
        assert_eq!(
            sort_updates2(&sorted, &(vec![97, 13, 75, 29, 47])),
            vec![97, 75, 47, 29, 13]
        );
    }
}
