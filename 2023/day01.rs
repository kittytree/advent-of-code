use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let input_file = "src/input.txt".to_string();
    let calibration_number = get_calibration_value(input_file);
    println!("{}", calibration_number);
}

fn get_calibration_value(input_file: String) -> u32 {
    let mut calibration_value = 0;
    let mut line_numbers = vec![];
    if let Ok(lines) = read_lines(input_file) {
        for line in lines.flatten() {
            line_numbers.push(get_first_number(&line));
            line_numbers.push(get_last_number(&line));
            calibration_value += give_calibration_value(&line_numbers);
            line_numbers.clear();
        }
    }
    calibration_value
}

fn get_first_number(line: &String) -> u32 {
    let mut first_number: u32 = 0;
    let mut first_index = usize::MAX;
    let mut digit_index = 1;
    for digit in DIGITS.iter() {
        let occurrences: Vec<_> = line.match_indices(*digit).collect();
        if occurrences.len() > 0 {
            if occurrences.first().unwrap().0 < first_index {
                first_index = occurrences.first().unwrap().0;
                first_number = digit_index;
            }
        }
        digit_index += 1;
    }
    for (index, char) in line.chars().enumerate() {
        if char.is_ascii_digit() {
            if index < first_index {
                first_number = char.to_digit(10).unwrap();
                first_index = index;
            }
        }
    }
    first_number
}
fn get_last_number(line: &String) -> u32 {
    let mut last_number: u32 = 0;
    let mut last_index = usize::MIN;
    let mut digit_index = 1;
    for digit in DIGITS.iter() {
        let occurrences: Vec<_> = line.match_indices(*digit).collect();
        if occurrences.len() > 0 {
            if occurrences.last().unwrap().0 > last_index {
                last_index = occurrences.last().unwrap().0;
                last_number = digit_index;
            }
        }
        digit_index += 1;
    }
    for (index, char) in line.chars().enumerate() {
        if char.is_ascii_digit() {
            if index > last_index {
                last_number = char.to_digit(10).unwrap();
                last_index = index;
            }
        }
    }
    last_number
}
fn give_calibration_value(line_numbers: &Vec<u32>) -> u32 {
    if line_numbers.len() == 0 {
        0
    } else {
        if line_numbers[1] == 0 {
            line_numbers[0] * 10 + line_numbers[0]
        } else {
            line_numbers[0] * 10 + line_numbers[1]
        }
    }
}
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
