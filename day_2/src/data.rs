use std::fs::File;
use std::io::{self, BufRead};

pub fn load_real_data() -> Vec<Vec<i32>> {
    load_matrix_text_file("/Users/Dino/projects/advent_of_code_2024/day_2/src/real_data.txt").unwrap()
}
pub fn load_test_data() -> Vec<Vec<i32>> {
    load_matrix_text_file("/Users/Dino/projects/advent_of_code_2024/day_2/src/test_data.txt").unwrap()
}

fn load_matrix_text_file(file_path: &str) -> io::Result<Vec<Vec<i32>>> {
   let mut first_dimension = Vec::new();

    // Open the file in read-only mode.
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let mut second_dimension = Vec::new();
        let line = line?;
        let data: Vec<&str> = line.split_whitespace().collect();
        for d in data {
            second_dimension.push(d.parse().unwrap())
        }
        first_dimension.push(second_dimension)
    }

    Ok(first_dimension)
}