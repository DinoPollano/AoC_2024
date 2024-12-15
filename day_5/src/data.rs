use std::fs::read_to_string;
use std::io::{self, BufRead};
use std::path::PathBuf;

pub fn load_test_data() -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("resources/data/test.txt");
    load_list_file(d.as_path().to_str().unwrap()).unwrap()
}

fn load_list_file(file_path: &str) -> io::Result<(Vec<(u32, u32)>, Vec<Vec<u32>>)> {
    let mut order_vec: Vec<(u32, u32)> = Vec::new();
    let mut update_vec: Vec<Vec<u32>> = Vec::new();
    let mut on_update_lists: bool = false;
    for line in read_to_string(file_path).unwrap().lines() {
        if line.is_empty() {
            on_update_lists = true;
        } else if on_update_lists {
            update_vec.push(line.split(',').map(|x| x.parse().unwrap()).collect());
        } else {
            let pair: Vec<&str> = line.split("|").collect();
            order_vec.push((pair[0].parse().unwrap(), pair[1].parse().unwrap()));
        }
    }

    Ok((order_vec, update_vec))
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;
    use crate::data::load_list_file;
    #[test]
    fn test_load_list_file() {
        let expected_order = [
            (47, 53),
            (97, 13),
            (97, 61),
            (97, 47),
            (75, 29),
            (61, 13),
            (75, 53),
            (29, 13),
            (97, 29),
            (53, 29),
            (61, 53),
            (97, 53),
            (61, 29),
            (47, 13),
            (75, 47),
            (97, 75),
            (47, 61),
            (75, 61),
            (47, 29),
            (75, 13),
            (53, 13),
        ];

        let expected_update = vec![
            vec![75, 47, 61, 53, 29],
            vec![97, 61, 53, 29, 13],
            vec![75, 29, 13],
            vec![75, 97, 47, 61, 53],
            vec![61, 13, 29],
            vec![97, 13, 75, 29, 47],
        ];

        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/data/test.txt");
        let (actual_order, actual_update) = load_list_file(d.as_path().to_str().unwrap()).unwrap();
        assert_eq!(actual_order, expected_order);
        assert_eq!(actual_update, expected_update);

    }
}
