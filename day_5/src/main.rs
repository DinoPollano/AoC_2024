mod data;
use std::collections::{HashMap, HashSet};

pub fn sort_pages(order_vec: Vec<(u32, u32)>) -> HashMap<u32, HashSet<u32>> {
    let mut order_map: HashMap<u32, HashSet<u32>> = HashMap::new();

    for (antecedant, subsequent) in order_vec {
        order_map.entry(antecedant).or_default().insert(subsequent);
        order_map.entry(subsequent).or_default();
    }
    for set in &order_map{
        println!("{} : {:?}", set.0, set.1);
    }
    order_map
}

pub fn check_pages_are_correct(sorted_map: &HashMap<u32, HashSet<u32>>, updates: &Vec<u32>) -> bool {
    let mut page = updates.iter().peekable();
    let mut correct: bool = true;
    while correct {
        let page_number = match page.peek(){
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
        // println!("tail: {:?}", tail);
        // println!("order: {:?}", order);
        correct = order.is_superset(&tail);
    }
    correct
}

use data::load_test_data;
fn main() {
    println!("DAY 5");
    let (order_vec, updates_vec) = load_test_data();

    println!("{:?}", sort_pages(order_vec));
}

#[cfg(test)]
mod tests {
    use crate::{check_pages_are_correct, data::load_test_data, sort_pages};
    #[test]
    fn test_check_pages_are_correct() {
        let (order_vec, updates_vec) = load_test_data();
        let sorted = sort_pages(order_vec );
        for (i, expected_result) in vec![true, true, true, false, false, false].iter().enumerate(){
        assert_eq!(check_pages_are_correct(&sorted, &updates_vec[i]), *expected_result, "{:?}",&updates_vec[i]);
        }
    }
}
