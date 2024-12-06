use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

fn main() {
    let time_start = Instant::now();
    let input_file = "src/input.txt".to_string();

    let page_ordering_rules: HashMap<u32, Vec<u32>>;
    let pages_to_produce: Vec<Vec<u32>>;

    let invalid_pages: Vec<Vec<u32>>;

    (page_ordering_rules, pages_to_produce) = get_production_rules_and_input(input_file);

    invalid_pages =
        print_part_one_and_return_invalid_pages(page_ordering_rules.clone(), pages_to_produce);
    let elapsed_time_to_part_one_complete = time_start.elapsed();

    print_part_two(page_ordering_rules, invalid_pages);
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

fn print_part_two(page_ordering_rules: HashMap<u32, Vec<u32>>,mut pages_to_produce: Vec<Vec<u32>>) {
    println!("part two");
    let mut encountered_rules: HashMap<u32, bool> = HashMap::new();
    let mut all_pages_valid = false;
    let mut valid_pages:Vec<Vec<u32>> = Vec::new();
    let mut valid_page = true;
    let mut page_is_not_valid = true;

    let mut vec_of_middles: Vec<u32> = Vec::new();

    let mut need_to_swap: HashMap<u32, u32> = HashMap::new();

    while pages_to_produce.len() > 0 {
        need_to_swap.clear();
        encountered_rules.clear();
        page_is_not_valid = true;
        let mut page = pages_to_produce.pop().unwrap();
        println!("{:?}", page);
        while page_is_not_valid {
            if need_to_swap.len() == 0 {
                let mut page_number_count = 0;
                for page_number in &page {
                    if page_ordering_rules.contains_key(&page_number) {
                        let page_number_rules = page_ordering_rules.get(&page_number).unwrap();
                        for rule in page_number_rules {
                            if encountered_rules.contains_key(rule) {
                                valid_page = false;
                                need_to_swap.insert(rule.clone(),page_number_count);
                            }
                        }
                    }
                    encountered_rules.insert(page_number.clone(), true);
                    page_number_count += 1;
                }
            }else{
                let mut count = 0;
                while count < page.clone().len() {
                    if need_to_swap.contains_key(page.get(count).unwrap()) {
                        let index_to_swap_to = need_to_swap.get(&page[count]).unwrap();
                        let index_to_swap_to_usize = usize::try_from(index_to_swap_to.clone()).unwrap();
                        page.swap(count, index_to_swap_to_usize);
                        println!("meow");
                    }
                    count += 1;
                }
                need_to_swap.clear();
                encountered_rules.clear();
                valid_page = false;
            }
            if valid_page {
                page_is_not_valid = false;
                vec_of_middles.push(page.get((page.len() - 1) / 2).unwrap().clone());
            }else{
                valid_page = true;
            }
        }
        if valid_pages.len() == pages_to_produce.len() {
            all_pages_valid = true;
        }
    }
    let mut sum_middles = 0;
    for middle in vec_of_middles {
        sum_middles += middle;
        println!("middle: {}", middle);
    }
    println!("Sum of middles: {}", sum_middles);

}

fn print_part_one_and_return_invalid_pages(
    page_ordering_rules: HashMap<u32, Vec<u32>>,
    pages_to_produce: Vec<Vec<u32>>,
) -> Vec<Vec<u32>> {
    let mut encountered_rules: HashMap<u32, bool> = HashMap::new();
    let mut vec_middle_numbers: Vec<u32> = Vec::new();
    let mut list_of_invalid_pages: Vec<Vec<u32>> = Vec::new();

    for page in pages_to_produce {
        let mut valid_page: bool = true;
        for page_number in page.clone() {
            if valid_page {
                print!("{} ", page_number);
                if page_ordering_rules.contains_key(&page_number) {
                    let page_number_rules = page_ordering_rules.get(&page_number).unwrap();
                    for rule in page_number_rules {
                        if encountered_rules.contains_key(rule) {
                            valid_page = false;
                        }
                    }
                }
                if valid_page {
                    encountered_rules.insert(page_number, true);
                }
            }
        }
        encountered_rules.clear();
        if valid_page {
            vec_middle_numbers.push(page.get((page.len() - 1) / 2).unwrap().clone());
        } else {
            list_of_invalid_pages.push(page);
        }
        valid_page = true;
        println!();
    }
    println!();

    let mut sum_valid_middle_number = 0;
    for middle_number in vec_middle_numbers {
        sum_valid_middle_number += middle_number;
    }
    println!("Total middle number: {}", sum_valid_middle_number);

    list_of_invalid_pages
}

fn get_production_rules_and_input(input_file: String) -> (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>) {
    let mut page_ordering_rules: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut pages_to_produce: Vec<Vec<u32>> = Vec::new();

    let mut finished_adding_rules: bool = false;

    if let Ok(lines) = read_lines(input_file) {
        for line in lines.flatten() {
            println!("{}", line);
            if line.is_empty() {
                finished_adding_rules = true;
            } else if !finished_adding_rules {
                let mut vec_of_rules: Vec<u32> = Vec::new();
                let page_rule = line.split("|").collect::<Vec<&str>>();

                let key: u32 = page_rule[0].trim().parse().unwrap();
                let value: u32 = page_rule[1].trim().parse().unwrap();

                if page_ordering_rules.contains_key(&key) {
                    let mut vec_of_rules: Vec<u32> = page_ordering_rules.get(&key).unwrap().clone();
                    vec_of_rules.push(value);
                    page_ordering_rules.insert(key, vec_of_rules);
                } else {
                    vec_of_rules.push(value);
                    page_ordering_rules.insert(key, vec_of_rules);
                }
            } else {
                let page = line.split(",").collect::<Vec<&str>>();
                let mut page_numbers: Vec<u32> = Vec::new();
                for entry in page {
                    page_numbers.push(entry.trim().parse::<u32>().unwrap());
                }
                pages_to_produce.push(page_numbers);
            }
        }
    }
    println!();
    (page_ordering_rules, pages_to_produce)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
