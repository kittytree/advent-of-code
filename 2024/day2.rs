use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let input_file = "src/input.txt".to_string();
    let reports: Vec<Vec<i32>>;
    reports = print_lines(input_file);
    print_number_of_valid_reports(reports.clone());
    println!();
    // I needed a hint of using brute force to solve part 2
    // was trying to do dynamic programming at first and failed :/
    print_number_of_valid_reports_using_dampener(reports.clone());
}

fn print_number_of_valid_reports_using_dampener(reports: Vec<Vec<i32>>) {
    let mut count_valid_reports = 0;

    let mut last_entry: i32 = 0;

    for report in reports {
        let mut is_first = true;
        let mut is_decreasing = true;
        let mut is_increasing = true;
        let mut is_within_one_to_three = true;

        let num_entries = report.len();
        let mut count_outside = 0;
        let mut count_inside = 0;
        let mut valid_loop = false;

        while count_outside < num_entries {
            for entry in report.clone() {
                if count_inside == count_outside {
                } else {
                    if is_first {
                        is_first = false;
                        last_entry = entry;
                    } else {
                        if ((entry - last_entry).abs() >= 1) && ((entry - last_entry).abs() <= 3) {
                            if is_increasing {
                                if last_entry - entry < 0 {
                                    is_increasing = false;
                                }
                            }
                            if is_decreasing {
                                if last_entry - entry > 0 {
                                    is_decreasing = false;
                                }
                            }
                        } else {
                            is_within_one_to_three = false;
                        }
                    }
                    last_entry = entry;
                }
                count_inside += 1;
            }
            if (is_decreasing || is_increasing) && is_within_one_to_three {
                valid_loop = true;
            }
            count_inside = 0;
            count_outside += 1;
            is_first = true;
            is_decreasing = true;
            is_increasing = true;
            is_within_one_to_three = true;
        }
        count_outside = 0;

        if valid_loop {
            count_valid_reports += 1;
        }
    }

    println!(
        "Found {} valid reports without dampeners",
        count_valid_reports
    );
}

fn print_number_of_valid_reports(reports: Vec<Vec<i32>>) {
    let mut count_valid_reports = 0;

    let mut last_entry: i32 = 0;

    for report in reports {
        let mut is_first = true;
        let mut is_decreasing = true;
        let mut is_increasing = true;
        let mut is_within_one_to_three = true;

        for entry in report {
            if is_first {
                is_first = false;
                last_entry = entry;
            } else {
                if ((entry - last_entry).abs() >= 1) && ((entry - last_entry).abs() <= 3) {
                    if is_increasing {
                        if last_entry - entry < 0 {
                            is_increasing = false;
                        }
                    }
                    if is_decreasing {
                        if last_entry - entry > 0 {
                            is_decreasing = false;
                        }
                    }
                } else {
                    is_within_one_to_three = false;
                }
            }
            last_entry = entry;
        }
        if (is_decreasing || is_increasing) && is_within_one_to_three {
            count_valid_reports += 1;
        }
    }

    println!(
        "Found {} valid reports without dampeners",
        count_valid_reports
    );
}

fn print_lines(input_file: String) -> Vec<Vec<i32>> {
    let mut reports: Vec<Vec<i32>> = Vec::new();
    let mut report: Vec<i32> = Vec::new();
    if let Ok(lines) = read_lines(input_file) {
        for line in lines.flatten() {
            let split_line = line.split(" ").collect::<Vec<&str>>();
            for item in split_line {
                report.push(item.parse::<i32>().unwrap());
            }
            reports.push(report);
            report = Vec::new();
        }
    }
    reports
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
