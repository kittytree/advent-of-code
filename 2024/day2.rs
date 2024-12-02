use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let input_file = "src/input3.txt".to_string();
    let reports: Vec<Vec<i32>>;
    reports = print_lines(input_file);
    print_number_of_valid_reports(reports.clone());
    print_number_of_valid_reports_using_dampener(reports.clone());
}

fn print_number_of_valid_reports_using_dampener(reports: Vec<Vec<i32>>) {
    let mut count_valid_reports = 0;
    let mut count_bad_entries = 0;
    let mut last_entry: i32 = 0;

    for report in reports {
        let mut is_first = true;
        let mut is_decreasing = true;
        let mut is_increasing = true;
        let mut bad_entry_lock = false;
        let mut num_bad_increases = 0;
        let mut num_bad_decreases = 0;

        for entry in report {
            print!("{} ", entry);
            if is_first {
                is_first = false;
                last_entry = entry;
            }else{
                if ((entry - last_entry).abs() >= 1) && ((entry - last_entry).abs() <= 3 ) {
                    if is_increasing {
                        if last_entry - entry < 0 {
                            if count_bad_entries < 1 && num_bad_increases > 0{
                                num_bad_increases += 1;
                                count_bad_entries += 1;
                                if count_bad_entries != 1 {
                                    bad_entry_lock = true;
                                }
                            }else {
                                is_increasing = false;
                            }
                        }
                    }
                    if is_decreasing && !bad_entry_lock {
                        if last_entry - entry > 0 {
                            if count_bad_entries < 1 && num_bad_increases > 0{
                                num_bad_decreases += 1;
                                count_bad_entries += 1;
                                if count_bad_entries != 1 {
                                    bad_entry_lock = true;
                                }
                            }else {
                                is_decreasing = false;
                            }
                        }
                    }
                }else {
                    bad_entry_lock = true;
                    count_bad_entries += 1;
                }
            }
            if !bad_entry_lock {
                last_entry = entry;
            }
            bad_entry_lock = false;

        }
        if (is_decreasing || is_increasing) && count_bad_entries <= 2 {
            count_valid_reports += 1;
            println!(" and {} bad entries", count_bad_entries);
        }else {
            println!()
        }
        count_bad_entries = 0;
    }

    println!("Found {} valid reports using dampeners", count_valid_reports);

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

    println!("Found {} valid reports without dampeners", count_valid_reports);
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
