mod data;

use crate::data::load_actual_data;
use regex::Regex;

fn main() {
    println!("DAY 3");
    let input = load_actual_data().clone();
    let mut mul_ops: Vec<&str> = Vec::new();
    let mut res: Vec<&str> = Vec::new();
    let mut disabled: bool = false;
    for line in input.as_slice() {
        (res, disabled) = exctact_acceptalbe_mul_operators(&line.as_str(), disabled);
        mul_ops.append(&mut res);
    }
    let integers = extract_mul_integers(mul_ops);
    let result = mult_sum(integers);
    println!("result is: {}", result)
}

fn extract_mul_operators(line: &str) -> Vec<&str> {
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let mut output: Vec<&str> = Vec::new();
    let matches: Vec<&str> = re.find_iter(line).map(|m| m.as_str()).collect();
    for m in matches {
        output.push(&m);
    }
    output
}

fn exctact_acceptalbe_mul_operators(line: &str, mut disabled: bool) -> (Vec<&str>, bool) {
    let re = Regex::new(r"(mul\((\d+),(\d+)\))|don't\(\)|do\(\)").unwrap();
    let mut output: Vec<&str> = Vec::new();
    let mut thing: String = String::new();
    for m in re.find_iter(line) {
        thing = m.as_str().to_string();

        match m.as_str() {
            "don't()" => disabled = true,
            "do()" => disabled = false,
            _ if !disabled && m.as_str().contains("mul") => {
                output.push(&m.as_str());
            }
            _ => {}
        }
    }
    (output, disabled)
}

fn extract_mul_integers(ops: Vec<&str>) -> Vec<(i32, i32)> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut output: Vec<(i32, i32)> = Vec::new();

    for op in ops {
        let (first, second): (String, String) = re
            .captures_iter(op)
            .map(|caps| {
                let (_, [first, second]) = caps.extract();
                (first, second)
            })
            .collect();
        let first_int = first.parse::<i32>().unwrap();
        let second_int = second.parse::<i32>().unwrap();
        output.push((first_int, second_int));
    }
    output
}

fn mult_sum(input: Vec<(i32, i32)>) -> i32 {
    input.iter().map(|(x, y)| x * y).sum()
}

#[cfg(test)]
mod tests {
    use crate::{
        data, exctact_acceptalbe_mul_operators, extract_mul_integers, extract_mul_operators,
        mult_sum,
    };

    #[test]
    fn test_regex_extraction() {
        // layout
        let input = data::load_test_data()[0].clone();

        let extract_output = extract_mul_operators(&input);
        assert_eq!(
            extract_output,
            vec!("mul(2,4)", "mul(5,5)", "mul(11,8)", "mul(8,5)")
        );
    }

    #[test]
    fn test_regex_extraction_to_integers() {
        // layout
        let input = vec!["mul(2,4)", "mul(5,5)", "mul(11,8)", "mul(8,5)"];

        let extract_output = extract_mul_integers(input);
        assert_eq!(extract_output, vec!((2, 4), (5, 5), (11, 8), (8, 5)));
    }

    #[test]
    fn test_multiply_and_sum() {
        let input = vec![(2, 4), (5, 5), (11, 8), (8, 5)];
        assert_eq!(mult_sum(input), 161);
    }

    #[test]
    fn test_regex_extract_mul_opr_without_dont() {
        let input = String::from(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        );
        let expected = vec!["mul(2,4)", "mul(8,5)"];
        let (result, _) = exctact_acceptalbe_mul_operators(&input, false);
        assert_eq!(expected, result);

        let input2 = String::from(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)adfasdfdont()asdfasd+mul(32,64](mul(11,8)undo()?mul(8,5))",
        );

        let expected2 = vec!["mul(2,4)", "mul(8,5)"];
        let (result2, _) = exctact_acceptalbe_mul_operators(&input2, false);
        assert_eq!(expected2, result2);

        let input3 = String::from(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)do+mul(32,64](mul(11,8)undo()?112319-192391029312-9do()1231231909qweqwfqj[]()while)_mul(8,5))while(3,5)",
        );

        let expected3 = vec!["mul(2,4)", "mul(8,5)"];
        let (result3, _) = exctact_acceptalbe_mul_operators(&input3, false);
        assert_eq!(expected3, result3);
    }

    #[test]
    fn test_part_2() {
        let input = String::from(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        );

        let mut mul_ops: Vec<&str> = Vec::new();
        let (mut res, _) = exctact_acceptalbe_mul_operators(input.as_str(), false);
        mul_ops.append(&mut res);

        let integers = extract_mul_integers(mul_ops);
        let result = mult_sum(integers);
        assert_eq!(result, 48)
    }
}
