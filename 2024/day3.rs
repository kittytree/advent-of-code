/*
     - use "$ cargo add regex" if your ide doesn't auto add regex from use regex::Regex
        to your Cargo.toml
*/

use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let input_file = "src/input2.txt".to_string();
    print_part_one_answer(input_file.clone());
    print_part_two_answer(input_file.clone());
}

fn print_part_two_answer(input_file: String) {
    let mut sum_multiplications = 0;
    let mut in_do: bool = true;
    if let Ok(lines) = read_lines(input_file) {
        for line in lines.flatten() {
            let re =
                Regex::new(r"mul\(\d+,\d+\)|don't\(\)|do\(\)").expect("Failed to compile regex");
            let result: Vec<&str> = re.find_iter(line.as_str()).map(|m| m.as_str()).collect();
            for item in result {
                if item.to_string() == "do()" {
                    in_do = true;
                } else if item.to_string() == "don't()" {
                    in_do = false;
                } else {
                    if in_do {
                        let stripped_prefix = item.strip_prefix("mul(").unwrap();
                        let isolated_inside = stripped_prefix.strip_suffix(")").unwrap();
                        let vec_multiply_arguments =
                            isolated_inside.split(",").collect::<Vec<&str>>();
                        println!(
                            "{} x {}",
                            vec_multiply_arguments[0], vec_multiply_arguments[1]
                        );
                        sum_multiplications +=
                            vec_multiply_arguments[0].trim().parse::<i32>().unwrap()
                                * vec_multiply_arguments[1].trim().parse::<i32>().unwrap();
                    }
                }
            }
        }
    }
    println!("Sum of multiplications: {}", sum_multiplications);
}

fn print_part_one_answer(input_file: String) {
    let mut sum_multiplications = 0;
    if let Ok(lines) = read_lines(input_file) {
        for line in lines.flatten() {
            let re = Regex::new(r"mul\(\d+,\d+\)").expect("Failed to compile regex");
            let result: Vec<&str> = re.find_iter(line.as_str()).map(|m| m.as_str()).collect();
            for item in result {
                let stripped_prefix = item.strip_prefix("mul(").unwrap();
                let isolated_inside = stripped_prefix.strip_suffix(")").unwrap();
                let vec_multiply_arguments = isolated_inside.split(",").collect::<Vec<&str>>();
                println!(
                    "{} x {}",
                    vec_multiply_arguments[0], vec_multiply_arguments[1]
                );
                sum_multiplications += vec_multiply_arguments[0].trim().parse::<i32>().unwrap()
                    * vec_multiply_arguments[1].trim().parse::<i32>().unwrap();
            }
        }
    }
    println!("Sum of multiplications: {}", sum_multiplications);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
