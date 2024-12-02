use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let input_file = "src/input.txt".to_string();
    let reports: Vec<Vec<i32>>;
    reports = print_lines(input_file);
    print_number_of_valid_reports(reports);
}

fn print_number_of_valid_reports(reports: Vec<Vec<i32>>) {
    let mut count_valid_reports = 0;

    let mut last_entry: i32 = 0;
    let mut is_first = true;

    let mut is_decreasing: bool = true;
    let mut is_increasing: bool = true;
    let mut is_within_one_to_three: bool = true;

    for report in reports {
        is_first = true;
        is_decreasing = true;
        is_increasing = true;
        is_within_one_to_three = true;

        for entry in report {
            print!("{} ", entry);
            if is_first {
                is_first = false;
                last_entry = entry;
            }else{
                if ((entry - last_entry).abs() >= 1) && ((entry - last_entry).abs() <= 3 ) {
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
                }else {
                    is_within_one_to_three = false;
                }
            }
            last_entry = entry;
        }
        if (is_decreasing || is_increasing) && is_within_one_to_three {
            count_valid_reports += 1;
        }
        println!();
    }

    println!("Found {} valid reports", count_valid_reports);
}

fn print_lines(input_file: String) -> Vec<Vec<i32>> {
    let mut reports:Vec<Vec<i32>> = Vec::new();
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
