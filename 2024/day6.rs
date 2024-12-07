/*
    - starting code I use for each day. On each day i start a new cargo project
    - I know this can be streamlined better but I'm still learning :)
    - maybe after i get stuck on a day ill take a day off to re-structure this stuff to be better
*/

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

fn main() {
    let time_start = Instant::now();
    let input_file = "src/input.txt".to_string();

    get_starting_hashes(input_file);

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

fn get_starting_hashes(input_file: String) {
    if let Ok(lines) = read_lines(input_file) {
        for line in lines.flatten() {
            for character in line.chars() {
                if character != '.' && character != '#' {
                    println!("{}", character);
                }
            }
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
