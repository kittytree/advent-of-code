use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

fn main() {
    let time_start = Instant::now();
    let input_file = "src/input.txt".to_string();
    let grid_map: HashMap<(i32, i32), String>;
    let max_row: i32;
    let max_col: i32;
    (grid_map, max_row, max_col) = get_grid_map(input_file);
    print_part_one_xmas_matches(grid_map.clone(), max_row, max_col);
    let elapsed_time_to_part_one_complete = time_start.elapsed();
    print_part_two_x_mas_matches(grid_map.clone(), max_row, max_col);
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

fn print_part_two_x_mas_matches(grid_map: HashMap<(i32, i32), String>, max_row: i32, max_col: i32) {
    let mut row: i32 = 0;
    let mut col: i32 = 0;
    let mut num_matches = 0;

    while row < max_row {
        while col < max_col {
            match grid_map.get(&(row, col)).cloned() {
                Some(character) => {
                    if character == "A" {
                        println!("A found at ({}, {})", row, col);
                        if return_x_mas_match_check(grid_map.clone(), row, col) {
                            num_matches += 1;
                            println!("matched X-MAS on A here ({},{})", row, col);
                        }
                    }
                }
                None => {}
            }
            col += 1;
        }
        col = 0;
        row += 1;
    }
    println!("Number of matches: {}", num_matches);
}

fn return_x_mas_match_check(grid_map: HashMap<(i32, i32), String>, row: i32, col: i32) -> bool {
    let down_right_diagonal = check_mas_from_3_coords(
        grid_map.clone(),
        (row - 1, col - 1),
        (row, col),
        (row + 1, col + 1),
    );

    let down_right_diagonal_reversed = check_mas_from_3_coords(
        grid_map.clone(),
        (row + 1, col + 1),
        (row, col),
        (row - 1, col - 1),
    );

    if !(down_right_diagonal_reversed || down_right_diagonal) {
        return false;
    }

    let down_left_diagonal = check_mas_from_3_coords(
        grid_map.clone(),
        (row + 1, col - 1),
        (row, col),
        (row - 1, col + 1),
    );

    let down_left_diagonal_reversed = check_mas_from_3_coords(
        grid_map.clone(),
        (row - 1, col + 1),
        (row, col),
        (row + 1, col - 1),
    );

    if !(down_left_diagonal_reversed || down_left_diagonal) {
        return false;
    }
    true
}

fn print_part_one_xmas_matches(grid_map: HashMap<(i32, i32), String>, max_row: i32, max_col: i32) {
    let mut row: i32 = 0;
    let mut col: i32 = 0;
    let mut num_matches = 0;
    while row < max_row {
        while col < max_col {
            match grid_map.get(&(row, col)).cloned() {
                Some(character) => {
                    if character == "X" {
                        println!("X found at ({},{})", row, col);
                        let num_matches_on_x = return_max_match_check(grid_map.clone(), row, col);
                        if num_matches_on_x != 0 {
                            println!("matched XMAS on X here ({},{})", row, col);
                            num_matches += num_matches_on_x;
                        }
                    }
                }
                None => {}
            }
            col += 1;
        }
        col = 0;
        row += 1;
    }
    println!("Number of matches: {}", num_matches);
}

fn return_max_match_check(grid_map: HashMap<(i32, i32), String>, row: i32, col: i32) -> u32 {
    let mut num_matches = 0;
    let right_match = check_mas_from_3_coords(
        grid_map.clone(),
        (row, col + 1),
        (row, col + 2),
        (row, col + 3),
    );
    if right_match {
        num_matches += 1;
    }
    let left_match = check_mas_from_3_coords(
        grid_map.clone(),
        (row, col - 1),
        (row, col - 2),
        (row, col - 3),
    );
    if left_match {
        num_matches += 1;
    }
    let up_match = check_mas_from_3_coords(
        grid_map.clone(),
        (row - 1, col),
        (row - 2, col),
        (row - 3, col),
    );
    if up_match {
        num_matches += 1;
    }
    let down_match = check_mas_from_3_coords(
        grid_map.clone(),
        (row + 1, col),
        (row + 2, col),
        (row + 3, col),
    );
    if down_match {
        num_matches += 1;
    }
    let left_up_match = check_mas_from_3_coords(
        grid_map.clone(),
        (row - 1, col - 1),
        (row - 2, col - 2),
        (row - 3, col - 3),
    );
    if left_up_match {
        num_matches += 1;
    }
    let right_up_match = check_mas_from_3_coords(
        grid_map.clone(),
        (row + 1, col - 1),
        (row + 2, col - 2),
        (row + 3, col - 3),
    );
    if right_up_match {
        num_matches += 1;
    }
    let left_down_match = check_mas_from_3_coords(
        grid_map.clone(),
        (row - 1, col + 1),
        (row - 2, col + 2),
        (row - 3, col + 3),
    );
    if left_down_match {
        num_matches += 1;
    }
    let right_down_match = check_mas_from_3_coords(
        grid_map.clone(),
        (row + 1, col + 1),
        (row + 2, col + 2),
        (row + 3, col + 3),
    );
    if right_down_match {
        num_matches += 1;
    }
    num_matches
}

fn check_mas_from_3_coords(
    grid_map: HashMap<(i32, i32), String>,
    coord_1: (i32, i32),
    coord_2: (i32, i32),
    coord_3: (i32, i32),
) -> bool {
    match grid_map.get(&(coord_1.0, coord_1.1)) {
        Some(coord_1_char) => {
            if coord_1_char.eq("M") {
                match grid_map.get(&(coord_2.0, coord_2.1)) {
                    Some(coord_2_char) => {
                        if coord_2_char.eq("A") {
                            match grid_map.get(&(coord_3.0, coord_3.1)) {
                                Some(coord_3_char) => {
                                    if coord_3_char.eq("S") {
                                        true
                                    } else {
                                        false
                                    }
                                }
                                None => false,
                            }
                        } else {
                            false
                        }
                    }
                    None => false,
                }
            } else {
                false
            }
        }
        None => false,
    }
}

fn get_grid_map(input_file: String) -> (HashMap<(i32, i32), String>, i32, i32) {
    let mut grid_map: HashMap<(i32, i32), String> = HashMap::new();
    let mut row = 0;
    let mut col = 0;
    let mut max_col = 0;
    if let Ok(lines) = read_lines(input_file) {
        for line in lines.flatten() {
            for char in line.chars() {
                print!("{}", char);
                grid_map.insert((row, col), char.to_string());
                col += 1;
            }
            println!();
            max_col = col;
            col = 0;
            row += 1;
        }
    }
    (grid_map, row, max_col)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
