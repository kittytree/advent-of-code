/*
INCOMPLETE
WENT DOWN WRONG RABBIT HOLE WITH STRUCT TYPING
*/

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct PartNumber {
    part_builder: Vec<char>,
    part_number: u32,
    part_coordinates: Vec<(u32, u32)>,
}

struct Symbol {
    symbol_coordinate: (u32, u32),
}

struct Engine {
    all_parts: Vec<PartNumber>,
}

struct Schematic {
    all_symbols: Vec<Symbol>,
}

impl PartNumber {
    fn new() -> PartNumber {
        return PartNumber {
            part_builder: Vec::new(),
            part_number: 0,
            part_coordinates: Vec::new(),
        };
    }

    fn set_part_number(&mut self, part_number: u32) {
        self.part_number = part_number;
    }

    fn add_coordinate(&mut self, coordinate: (u32, u32)) {
        self.part_coordinates.push(coordinate);
    }

    fn add_part(&mut self, part: char) {
        self.part_builder.push(part);
    }

    fn combine_parts(&mut self) {
        self.part_number = self
            .part_builder
            .clone()
            .into_iter()
            .collect()
            .parse::<u32>()
            .unwrap();
    }
}

impl Symbol {
    fn new(row: u32, col: u32) -> Symbol {
        return Symbol {
            symbol_coordinate: (row, col),
        }
    }
}

impl Engine {
    fn new() -> Engine {
        return Engine {
            all_parts: Vec::new(),
        };
    }

    fn add_part(&mut self, part: PartNumber) {
        self.all_parts.push(part);
    }
}

impl Schematic {
    fn new() -> Schematic {
        return Schematic {
            all_symbols: Vec::new(),
        };
    }
    fn add_symbol(&mut self, symbol: Symbol) {
        self.all_symbols.push(symbol)
    }
}

fn main() {
    let input_file = "src/input2.txt".to_string();
    println!("Reading file: {}", input_file);
    read_board(input_file);
}

fn read_board(input_file: String) {
    let mut row = 0;
    let mut col = 0;

    let mut part_switch: bool = false;

    let mut schematic = Schematic::new();
    let mut engine = Engine::new();
    let mut part = PartNumber::new();

    if let Ok(lines) = read_lines(input_file) {
        for line in lines {
            for char in line.unwrap().chars() {
                if char.is_ascii_digit() && !part_switch {
                    part_switch = true;
                    part.add_part(char);
                    part.add_coordinate((row, col));
                }else {
                    part_switch = false;
                    engine.add_part(part);
                    part = PartNumber::new();
                    if !char.eq(&'.') {
                        let symbol = Symbol::new(row, col);
                        schematic.add_symbol(symbol);
                    }
                }
                //println!("{}", char);
                col += 1;
                //println!("{}", col);
            }
            row += 1;
            col = 0;
            //println!("row: {}", row);
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
