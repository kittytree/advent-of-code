/*
Failed day 3 part 2, will return in future. Gonna move on so I can continue to learn more.
*/

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(PartialEq, Eq, Hash)]
struct PartNumber {
    part_builder: String,
    part_number: u32,
    part_coordinates: Vec<(u32, u32)>,
}

struct Symbol {
    char_symbol: char,
    symbol_coordinate: (u32, u32),
    gear_ratio: u32,
    gears: HashMap<PartNumber, bool>,
}

struct Engine {
    all_parts: Vec<PartNumber>,
}
struct Schematic {
    all_symbols: Vec<Symbol>,
}

#[allow(dead_code)]
impl PartNumber {
    fn new() -> PartNumber {
        return PartNumber {
            part_builder: String::from(""),
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
        self.part_number = self.part_builder.parse::<u32>().unwrap();
    }

    fn is_null(&self) -> bool {
        String::is_empty(&self.part_builder)
    }
}

impl Symbol {
    fn new(row: u32, col: u32, char: char) -> Symbol {
        return Symbol {
            char_symbol: char,
            symbol_coordinate: (row, col),
            gear_ratio: 0,
            gears: HashMap::new(),
        };
    }

    fn add_gear_ratio(&mut self, gear_ratio: u32) {
        self.gear_ratio += 1;
    }

    fn add_gear(&mut self, gear: PartNumber) {
        self.gears.insert(gear, true);
    }

    fn does_gear_exist(&mut self, gear: PartNumber) -> bool {
        return self.gears.contains_key(&gear);
    }
}
#[allow(dead_code)]
impl Engine {
    fn new() -> Engine {
        return Engine {
            all_parts: Vec::new(),
        };
    }

    fn add_part(&mut self, part: PartNumber) {
        self.all_parts.push(part);
    }

    fn print_engine(&self) {
        for part in self.all_parts.iter() {
            println!("{:?}", part.part_number);
            println!("{:?}", part.part_coordinates);
        }
    }
}

#[allow(dead_code)]
impl Schematic {
    fn new() -> Schematic {
        return Schematic {
            all_symbols: Vec::new(),
        };
    }
    fn add_symbol(&mut self, symbol: Symbol) {
        self.all_symbols.push(symbol)
    }

    fn print_schematic(&self) {
        for symbol in self.all_symbols.iter() {
            println!("{:?}", symbol.symbol_coordinate);
        }
    }
}

fn main() {
    let input_file = "src/input.txt".to_string();
    println!("Reading file: {}", input_file);
    let (schematic, engine) = generate_board(input_file);

    let part_neighbours_part_1 = get_hash_of_part_neighbours_part_1(&schematic);
    sum_part_neighbours_part_1(&engine, &part_neighbours_part_1);

    let mut part_neighbours_part_2 = get_hash_of_part_neighbours_part_2(&schematic);
    sum_part_neighbours_part_2(&engine, part_neighbours_part_2.clone());
}

fn sum_part_neighbours_part_2(
    engine: &Engine,
    mut vec_part_neighbors: Vec<((u32, u32), Vec<(u32, u32)>, u32)>,
) {
    let mut temp_ratio = 1;
    let mut total_ratio = 0;
    let mut gear_count_is_two = false;
    let mut first_gear = 0;
    let mut second_gear = 0;

    let mut temp_stack_symbols = Vec::new();

    let mut hash_star_touches: HashMap<(u32, u32), u32> = HashMap::new();

    let mut switch_match = false;

    for star in vec_part_neighbors.iter() {
        println!("{:?}", star.0);
    }

    for part in engine.all_parts.iter() {
        temp_stack_symbols.clear();
        for part_coordinate in part.part_coordinates.iter() {
            for star in vec_part_neighbors.clone() {
                for star_neighbour in star.1.iter() {
                    if star_neighbour == part_coordinate {
                        //switch_match = true;
                        if !temp_stack_symbols.contains(&star.0) {
                            temp_stack_symbols.push(star.0);
                            for item in temp_stack_symbols.iter() {
                                println!("{:?}", item);
                            }
                            if hash_star_touches.contains_key(&star.0) {
                                if hash_star_touches.get(&star.0).unwrap().clone() <= 1 {
                                    second_gear = part.part_number;
                                    hash_star_touches.remove(&star.0);
                                    hash_star_touches.insert(star.0, 2);
                                    //switch_match = true;
                                    gear_count_is_two = true;
                                    println!("meow 2 {}@ {:?}", part.part_number, part.part_coordinates);
                                } else {
                                    hash_star_touches.remove(&star.0);
                                    hash_star_touches.insert(star.0, 100);
                                    gear_count_is_two = false;
                                    //switch_match = true;
                                    //println!("meow 3 {}", part.part_number);
                                }
                            } else {
                                hash_star_touches.insert(star.0, 1);
                                gear_count_is_two = false;
                                second_gear = 0;
                                first_gear = part.part_number;
                                //switch_match = true;
                                println!("meow 1 {} @ {:?}", part.part_number, part.part_coordinates);
                            }
                        }
                    }
                    if switch_match {
                        //switch_match = false;
                        //println!("meow brok");
                        break;
                    }
                    //switch_match = false;
                }
                if (gear_count_is_two) {
                    println!("meow gear count is 2 ");
                    println!(
                        "symbol {:?}, first_gear {}, second_gear {}",
                        &star.0, first_gear, second_gear
                    );
                    total_ratio += first_gear * second_gear;
                    gear_count_is_two = false;
                }
                //switch_match = false;
            }
        }
    }



    println!("Total ratio: {}", total_ratio);
}

fn get_hash_of_part_neighbours_part_2(
    schematic: &Schematic,
) -> Vec<((u32, u32), Vec<(u32, u32)>, u32)> {
    let mut vec_part_neighbours: Vec<((u32, u32), Vec<(u32, u32)>, u32)> = Vec::new();
    let mut vec_of_neighbours: Vec<(u32, u32)> = Vec::new();
    for part in schematic.all_symbols.iter() {
        if part.char_symbol.eq(&'*') {
            vec_of_neighbours.push(part.symbol_coordinate);
            vec_of_neighbours.push((part.symbol_coordinate.0 - 1, part.symbol_coordinate.1 - 1));
            vec_of_neighbours.push((part.symbol_coordinate.0, part.symbol_coordinate.1 - 1));
            vec_of_neighbours.push((part.symbol_coordinate.0 + 1, part.symbol_coordinate.1 - 1));
            vec_of_neighbours.push((part.symbol_coordinate.0 - 1, part.symbol_coordinate.1));
            vec_of_neighbours.push((part.symbol_coordinate.0, part.symbol_coordinate.1));
            vec_of_neighbours.push((part.symbol_coordinate.0 + 1, part.symbol_coordinate.1));
            vec_of_neighbours.push((part.symbol_coordinate.0 - 1, part.symbol_coordinate.1 + 1));
            vec_of_neighbours.push((part.symbol_coordinate.0, part.symbol_coordinate.1 + 1));
            vec_of_neighbours.push((part.symbol_coordinate.0 + 1, part.symbol_coordinate.1 + 1));

            vec_part_neighbours.push((part.symbol_coordinate, vec_of_neighbours.clone(), 0));
            vec_of_neighbours.clear();
        }
    }
    vec_part_neighbours
}
fn sum_part_neighbours_part_1(
    engine: &Engine,
    hash_map_part_neighbours: &HashMap<(u32, u32), bool>,
) {
    let mut sum_neighbours = 0;
    for part in engine.all_parts.iter() {
        for part_coordinate in part.part_coordinates.iter() {
            if hash_map_part_neighbours.contains_key(&part_coordinate) {
                sum_neighbours += part.part_number;
                break;
            }
        }
    }
    println!("Sum neighbours part 1: {}", sum_neighbours);
}

fn get_hash_of_part_neighbours_part_1(schematic: &Schematic) -> HashMap<(u32, u32), bool> {
    let mut hash_map_part_neighbours = HashMap::new();
    for part in schematic.all_symbols.iter() {
        hash_map_part_neighbours.insert(part.symbol_coordinate, true);
        hash_map_part_neighbours.insert(
            (part.symbol_coordinate.0 - 1, part.symbol_coordinate.1 - 1),
            true,
        );

        hash_map_part_neighbours.insert(
            (part.symbol_coordinate.0 - 1, part.symbol_coordinate.1),
            true,
        );

        hash_map_part_neighbours.insert(
            (part.symbol_coordinate.0 - 1, part.symbol_coordinate.1 + 1),
            true,
        );

        hash_map_part_neighbours.insert(
            (part.symbol_coordinate.0, part.symbol_coordinate.1 - 1),
            true,
        );

        hash_map_part_neighbours.insert(
            (part.symbol_coordinate.0, part.symbol_coordinate.1 + 1),
            true,
        );

        hash_map_part_neighbours.insert(
            (part.symbol_coordinate.0 + 1, part.symbol_coordinate.1 - 1),
            true,
        );

        hash_map_part_neighbours.insert(
            (part.symbol_coordinate.0 + 1, part.symbol_coordinate.1),
            true,
        );

        hash_map_part_neighbours.insert(
            (part.symbol_coordinate.0 + 1, part.symbol_coordinate.1 + 1),
            true,
        );
    }
    hash_map_part_neighbours
}

fn generate_board(input_file: String) -> (Schematic, Engine) {
    let mut row = 0;
    let mut col = 0;

    let mut schematic = Schematic::new();
    let mut engine = Engine::new();
    let mut part = PartNumber::new();

    if let Ok(lines) = read_lines(input_file) {
        for line in lines {
            for char in line.unwrap().chars() {
                if char.is_ascii_digit() {
                    part.add_part(char);
                    part.add_coordinate((row, col));
                } else {
                    if !part.is_null() {
                        part.combine_parts();
                        engine.add_part(part);
                        part = PartNumber::new();
                    }
                    if !char.eq(&'.') {
                        let symbol = Symbol::new(row, col, char);
                        schematic.add_symbol(symbol);
                    }
                }
                col += 1;
            }
            row += 1;
            col = 0;
        }
    }
    return (schematic, engine);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
