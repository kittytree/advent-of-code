use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let input_file = "src/input.txt".to_string();
    let left_vector: Vec<i32> ;
    let right_vector: Vec<i32>;
    (left_vector, right_vector) = get_sorted_vectors(input_file);

    get_part_one_distance(&left_vector, &right_vector);

}

fn get_part_one_distance(left_vector: &Vec<i32>, right_vector: &Vec<i32>) {
    let mut count = 0;
    let mut total_distance = 0;
    while count < left_vector.len() && right_vector.len() > 0 {
        total_distance += (left_vector[count] - right_vector[count]).abs();
        println!("{}", total_distance);
        count += 1
    }
    println!("Total distance: {}", total_distance);
}

fn get_sorted_vectors(input_file: String)  -> (Vec<i32>, Vec<i32>){
    let mut left_vector: Vec<i32> = Vec::new();
    let mut right_vector: Vec<i32> = Vec::new();
    if let Ok(lines) = read_lines(input_file) {
        for line in lines.flatten() {
            let stripped_line = line.split("   ").collect::<Vec<&str>>();
            left_vector.push(stripped_line[0].parse::<i32>().unwrap());
            right_vector.push(stripped_line[1].parse::<i32>().unwrap());
        }
    }
    left_vector.sort();
    right_vector.sort();
    return (left_vector, right_vector)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
