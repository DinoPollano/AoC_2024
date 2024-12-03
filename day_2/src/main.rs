mod data;


fn main() {
    let data = data::load_real_data();
    println!("number of safe reports: {}", dampened_count_safe_readings(data));
}

fn gradient((x_1, y_1): (i32, i32), (x_2, y_2): (i32, i32)) -> f32 {
    (y_2 as f32 - y_1 as f32) / (x_2 as f32 - x_1 as f32)
}

fn count_safe_readings(data: Vec<Vec<i32>>) -> i32 {
    let mut num_safe_readings = 0;
    for report in data {
        if check_report_safety(&report) { num_safe_readings += 1 };
    }
    num_safe_readings
}

fn dampened_count_safe_readings(data: Vec<Vec<i32>>) -> i32 {
    let mut num_safe_readings = 0;
    for report in data {
        if check_report_safety(&report) {
            num_safe_readings += 1;
        }
        else {
            for i in 0 ..report.len() {
                let mut clon = report.clone();
                _ = clon.remove(i);
                if check_report_safety(&clon) {
                    num_safe_readings += 1;
                    break;
                }
            }
        }
    }
    num_safe_readings
}

fn check_report_safety(report: &Vec<i32>) -> bool {
    let mut is_safe: bool = true;
    let mut positive_direction: Option<bool> = None;
    for n in 1..report.len(){
        let gradient = gradient(((n) as i32, report[n]), ((n-1) as i32, report[n-1]));
        if (positive_direction != None && positive_direction.unwrap() != (gradient > 0.0)) || (gradient.abs() > 3.0 || gradient.abs() < 1.0) {
                is_safe = false;
                break;
        } else {
            let _ = positive_direction.insert(gradient > 0.0);
        }
    }
    is_safe
}

#[cfg(test)]
mod day_2_tests {
    use crate::data::load_test_data;
    use super::*;

    #[test]
    fn test_part1() {
        let data = load_test_data();
        let count = crate::count_safe_readings(data);
        assert_eq!(count, 2)
    }

    #[test]
    fn test_part2() {
        let data = load_test_data();
        let count = dampened_count_safe_readings(data);
        assert_eq!(count, 4)
    }

    #[test]
    fn test_single_report(){
        let  data: Vec<i32> = vec![1,2,3,4,5];
        assert_eq!(check_report_safety(&data), true);

        let  data2: Vec<i32> = vec![1,7,3,4,5];
        assert_eq!(check_report_safety(&data2), false);

        let  data3: Vec<i32> = vec![5,4,3,2,1];
        assert_eq!(check_report_safety(&data3), true);
    }

}