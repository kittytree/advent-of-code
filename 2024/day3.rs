/*
    - ToDO: Split on regex: mul\(\d+,\d+\)
     - add regex to your Cargo.toml if your ide doesn't on use
*/

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn main() {
    let input_file = "src/input2.txt".to_string();
    print_lines(input_file);

}

fn print_lines(input_file: String){
    if let Ok(lines) = read_lines(input_file) {
        for line in lines.flatten() {
            println!("{}", line);
            let re = Regex::new(r"mul\(\d+,\d+\)").expect("Failed to compile regex");
            let result = re.captures(line.as_str()).unwrap();
            for item in result {
                println!("{}", item);
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
