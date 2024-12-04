use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;

pub fn load_test_data() -> Vec<String> {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("resources/data/test.txt");

    println!("loading data from: {}", d.display());
    let result =  load_corrupted_text_file(d.as_path().to_str().unwrap());
    return result.unwrap()
}

pub fn load_actual_data() -> Vec<String> {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("resources/data/data.txt");

    println!("loading data from: {}", d.display());
    let result =  load_corrupted_text_file(d.as_path().to_str().unwrap());
    return result.unwrap()   
}

fn load_corrupted_text_file(file_path: &str) -> io::Result<Vec<String>> {
    let mut lines = Vec::new();
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    lines.extend(reader.lines().map(|l| l.unwrap()));
    Ok(lines)

}
