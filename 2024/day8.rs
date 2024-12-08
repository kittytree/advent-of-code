use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;
use petgraph::graphmap;

fn main() {
    let time_start = Instant::now();
    let input_file = "src/input.txt".to_string();

    print_input(input_file);

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

fn print_input(input_file: String) {
    if let Ok(lines) = read_lines(input_file) {
        for line in lines.flatten() {
            println!("{}", line);
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
