use std::fs::read_to_string;
use std::io::{self, BufRead};
use std::path::PathBuf;

pub fn load_test_data() -> Vec<Vec<char>> {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("resources/data/test.txt");

    println!("loading data from: {}", d.display());
    let result =  load_matrix_file(d.as_path().to_str().unwrap());
    return result.unwrap()
}

pub fn load_actual_data() -> Vec<Vec<char>> {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("resources/data/data.txt");

    println!("loading data from: {}", d.display());
    let result =  load_matrix_file(d.as_path().to_str().unwrap());
    return result.unwrap()
}

fn load_matrix_file(file_path: &str)-> io::Result<Vec<Vec<char>>>{
    let mut matrix = Vec::new();
    
    for line in read_to_string(file_path).unwrap().lines() {
        matrix.push(line.to_string().chars().collect());
    }

    Ok(matrix)
}