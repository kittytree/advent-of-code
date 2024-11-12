use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let input_file = "src/input2.txt".to_string();
    println!("Reading file: {}", input_file);
    if let Ok(lines) = read_lines(input_file) {
        for line in lines {
            println!("{}", line.unwrap());
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
