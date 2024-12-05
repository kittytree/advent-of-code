use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

fn main() {
    let time_start = Instant::now();
    let input_file = "src/input2.txt".to_string();

    let page_ordering_rules: HashMap<u32, Vec<u32>>;
    let pages_to_produce: Vec<Vec<u32>>;

    (page_ordering_rules, pages_to_produce) = get_production_rules_and_input(input_file);

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

fn get_production_rules_and_input(input_file: String) -> (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>) {
    let mut page_ordering_rules: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut pages_to_produce: Vec<Vec<u32>> = Vec::new();

    let mut finished_adding_rules: bool = false;

    if let Ok(lines) = read_lines(input_file) {
        for line in lines.flatten() {
            println!("{}", line);
            if line.is_empty() {
                finished_adding_rules = true;
            }
            if !finished_adding_rules {
                let mut vec_of_rules: Vec<u32> = Vec::new();
                let page_rule = line.split("|").collect::<Vec<&str>>();

                let key: u32 = page_rule[0].trim().parse().unwrap();
                let value: u32 = page_rule[1].trim().parse().unwrap();

                if page_ordering_rules.contains_key(&key) {
                    let mut vec_of_rules: Vec<u32> = page_ordering_rules.get(&key).unwrap().clone();
                    vec_of_rules.push(value);
                    page_ordering_rules.insert(key, vec_of_rules);
                } else {
                    vec_of_rules.push(value);
                    page_ordering_rules.insert(key, vec_of_rules);
                }
            }else{
                let page = line.split(",").collect::<Vec<&str>>();
                let mut page_numbers: Vec<u32> = Vec::new();
                for entry in page {
                    page_numbers.push(entry.trim().parse::<u32>().unwrap());
                }
                pages_to_produce.push(page_numbers);
            }
        }
    }

    (page_ordering_rules, pages_to_produce)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
