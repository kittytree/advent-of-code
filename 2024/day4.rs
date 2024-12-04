use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let input_file = "src/input2.txt".to_string();
    let grid_map: HashMap<(u32,u32),String> = get_grid_map(input_file);
}

fn get_grid_map(input_file: String) -> HashMap<(u32,u32),String>{
    let mut grid_map:HashMap<(u32,u32),String> = HashMap::new();
    if let Ok(lines) = read_lines(input_file) {
        for line in lines.flatten() {
            println!("{}", line);
        }
    }
    grid_map
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
