use std::fs::File;
use std::io::{self, BufRead};

pub fn load_data() -> (Vec<i32>, Vec<i32>) {
    let result =  read_two_column_file("/Users/Dino/projects/advent_of_code_2024/day_1/src/ids.txt");
    return result.unwrap()
}

pub fn  test_data() -> (Vec<i32>, Vec<i32>){
    let first = vec![3,4,2,1,3,3];
    let second = vec![4,3,5,3,9,3];
    return (first, second);
}
fn read_two_column_file(file_path: &str) -> io::Result<(Vec<i32>, Vec<i32>)> {
    let mut col1 = Vec::new();
    let mut col2 = Vec::new();

    // Open the file in read-only mode.
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let columns: Vec<&str> = line.split_whitespace().collect();

        if columns.len() == 2 {
            let num1: i32 = columns[0].parse().unwrap_or(0);
            let num2: i32 = columns[1].parse().unwrap_or(0);

            col1.push(num1);
            col2.push(num2);
        }
    }

    Ok((col1, col2))
}