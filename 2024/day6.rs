use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

fn main() {
    let time_start = Instant::now();
    let input_file = "src/input.txt".to_string();

    let row_count: i32;
    let col_count: i32;
    let starting_position:((i32, i32), String);
    let hash_of_obstacles: HashMap<(i32, i32), String>;

    (row_count, col_count, starting_position, hash_of_obstacles) = get_starting_hashes(input_file);

    print_part_one_traveled_distance(row_count, col_count, &starting_position, &hash_of_obstacles);
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

fn print_part_one_traveled_distance(row_count: i32, col_count: i32, starting_position: &((i32, i32), String), hash_of_obstacles: &HashMap<(i32, i32), String>) {
    println!("here")
}

fn get_starting_hashes(input_file: String) -> (i32, i32, ((i32, i32), String), HashMap<(i32, i32), String>) {
    let mut row = 0;
    let mut col = 0;
    let mut max_col = 0;

    let mut starting_position: ((i32, i32), String) = ((0, 0), String::new());
    let mut hash_of_obstacles: HashMap<(i32, i32), String> = HashMap::new();

    if let Ok(lines) = read_lines(input_file) {
        for line in lines.flatten() {
            for character in line.chars() {
                if character == '#' {
                    hash_of_obstacles.insert((row, col), character.to_string());
                } else if character != '.'{
                    starting_position = ((row, col), character.to_string());
                }
                col += 1;
            }
            row += 1;
            max_col = col;
            col = 0;
        }
    }
    (row, max_col, starting_position, hash_of_obstacles)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
