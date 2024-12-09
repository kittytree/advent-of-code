use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;
use petgraph::graphmap;

fn main() {
    let time_start = Instant::now();
    let input_file = "src/input2.txt".to_string();

    let row_count: i32;
    let col_count: i32;
    let antennas: HashMap<String, Vec<(i32, i32)>>;


    (row_count, col_count, antennas) = parse_inpute(input_file);

    print_part_one_answer(row_count, col_count, &antennas);
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

fn print_part_one_answer(row_count: i32, col_count: i32, antennas: &HashMap<String, Vec<(i32, i32)>>) {
    println!("row count: {}, col count {}", row_count, col_count);
    for hash in antennas.keys() {
        println!("key: {}", hash);
    }

    let mut current_row: i32 = 0;
    let mut current_col: i32 = 0;

    while current_row < row_count && current_col < col_count {
        while current_col < col_count {
            current_col += 1;
        }
        current_row += 1;

    }
}

fn parse_inpute(input_file: String) -> (i32, i32, HashMap<String, Vec<(i32, i32)>>){
    let mut antennas: HashMap<String, Vec<(i32, i32)>> = HashMap::new();
    let mut row: i32 = 0;
    let mut col: i32 = 0;
    let mut max_col: i32 = 0;


    if let Ok(lines) = read_lines(input_file) {
        for line in lines.flatten() {
            for character in line.chars() {
                if character != '.' {
                    if antennas.contains_key(&character.to_string()) {
                        let mut antenna_coords = antennas.get(&character.to_string()).unwrap().clone();
                        antenna_coords.push((row, col));
                        antennas.insert(character.to_string(), antenna_coords);
                    }
                }
                col += 1;
            }
            max_col = col;
            col = 0;
            row += 1;
        }
    }
    (row, max_col, antennas)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
