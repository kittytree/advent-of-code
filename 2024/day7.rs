use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

fn main() {
    let time_start = Instant::now();
    let input_file = "src/input.txt".to_string();

    let equations: Vec<(u64, Vec<u64>)>;

    equations = process_input(input_file);

    print_part_one_answer(&equations);
    let elapsed_time_to_part_one_complete = time_start.elapsed();

    let elapsed_time_to_part_two_complete = time_start.elapsed();
    println!(
        "Time to complete part one: {:.2?}",
        elapsed_time_to_part_one_complete
    );
    println!(
        "Time to complete part two: {:.2?}",
        elapsed_time_to_part_two_complete
    );
}

fn print_part_one_answer(equations: &Vec<(u64, Vec<u64>)>) {
    println!("Part One:");
    let mut sum_valid_equations = 0;
    for equation in equations {
        println!("{:?}", equation);
        let is_valid_equation = recursive_equation_finding(equation.0, equation.1.clone(), 0, 0);
        if is_valid_equation {
            println!("Valid Equation");
            sum_valid_equations += equation.0;
        }
    }
    println!("Sum of Valid Equations: {}", sum_valid_equations);
}

fn recursive_equation_finding(answer: u64, inputs: Vec<u64>, result: u64, index: usize) -> bool {
    match result == answer && index == inputs.len() {
        true => true,
        false => match index < inputs.len() {
            false => false,
            true => {
                let popped_input = inputs.get(index).unwrap();
                let plus_path: bool = recursive_equation_finding(
                    answer,
                    inputs.clone(),
                    result + popped_input,
                    index + 1,
                );
                match plus_path {
                    false => match index == 0 {
                        true => {
                            let mult_path = recursive_equation_finding(
                                answer,
                                inputs.clone(),
                                1 * popped_input,
                                index + 1,
                            );
                            match mult_path {
                                true => true,
                                false => {
                                    let result_as_string = result.to_string();
                                    let popped_input_as_string = popped_input.to_string();
                                    let concat_input = (result_as_string + &popped_input_as_string)
                                        .trim()
                                        .parse::<u64>()
                                        .unwrap();
                                    recursive_equation_finding(
                                        answer,
                                        inputs.clone(),
                                        concat_input,
                                        index + 1,
                                    )
                                }
                            }
                        }
                        false => {
                            let mult_path = recursive_equation_finding(
                                answer,
                                inputs.clone(),
                                result * popped_input,
                                index + 1,
                            );
                            match mult_path {
                                false => {
                                    let result_as_string = result.to_string();
                                    let popped_input_as_string = popped_input.to_string();
                                    let concat_input = (result_as_string + &popped_input_as_string)
                                        .trim()
                                        .parse::<u64>()
                                        .unwrap();
                                    recursive_equation_finding(
                                        answer,
                                        inputs.clone(),
                                        concat_input,
                                        index + 1,
                                    )
                                }
                                true => true,
                            }
                        }
                    },
                    true => true,
                }
            }
        },
    }
}

fn process_input(input_file: String) -> Vec<(u64, Vec<u64>)> {
    let mut equation: Vec<(u64, Vec<u64>)> = Vec::new();
    if let Ok(lines) = read_lines(input_file) {
        let mut answer: u64 = 0;
        let mut inputs: Vec<u64> = Vec::new();
        for line in lines.flatten() {
            let split_line = line.split(": ").collect::<Vec<&str>>();
            answer = split_line[0].parse::<u64>().unwrap();
            for x in split_line[1].split(" ") {
                let number = x.parse::<u64>().unwrap();
                inputs.push(number);
            }
            equation.push((answer, inputs));
            answer = 0;
            inputs = Vec::new();
        }
    }

    equation
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
