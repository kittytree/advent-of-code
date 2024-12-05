use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

fn main() {
    let time_start = Instant::now();
    let input_file = "src/input.txt".to_string();

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

fn get_production_rules_and_input(input_file: String) -> (HashMap<u32, Vec<u32>> , Vec<Vec<u32>>) {
    let mut page_ordering_rules: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut pages_to_produce: Vec<Vec<u32>> = Vec::new();

    let mut finished_adding_rules: bool = false;

    if let Ok(lines) = read_lines(input_file) {
        for line in lines.flatten() {
            println!("{}", line);
            if line.is_empty(){
                finished_adding_rules = true;
            }
            if !finished_adding_rules {
                
            }

        }
    }

    return (page_ordering_rules, pages_to_produce);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
