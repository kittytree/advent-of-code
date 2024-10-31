use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;
struct Reveal {
    green: u32,
    blue: u32,
    red: u32,
}
struct Game {
    game: Vec<Reveal>,
}

impl Reveal {
    fn new() -> Reveal {
        return Reveal {
            green: 0,
            blue: 0,
            red: 0,
        };
    }
    fn set_green(&mut self, val: u32) {
        self.green = val;
    }
    fn set_blue(&mut self, val: u32) {
        self.blue = val;
    }
    fn set_red(&mut self, val: u32) {
        self.red = val;
    }
    fn is_valid(&self) -> bool {
        if self.red > 12 {
            false
        } else if self.green > 13 {
            false
        } else if self.blue > 14 {
            false
        } else {
            true
        }
    }
}

impl Game {
    fn new() -> Game {
        return Game { game: Vec::new() };
    }
    fn add_reveal(&mut self, reveal: Reveal) {
        self.game.push(reveal);
    }
}

fn main() {
    let input_file = "src/input.txt".to_string();
    let games_from_input = games_from_input_file(input_file);
    println!("size of input {}", games_from_input.len());
    print_valid_id_sum(&games_from_input);
}
fn print_valid_id_sum(vec_of_games: &Vec<Game>) {
    let mut i = 1;
    let mut id_sum = 0;
    let mut game_valid: bool = true;
    for games in vec_of_games {
        games.game.iter().for_each(|reveal| {
            if reveal.is_valid() {
            } else {
                game_valid = false;
            }
        });
        if game_valid {
            id_sum += i;
        }
        game_valid = true;
        i += 1;
    }
    println!("id_sum: {}", id_sum);
}
fn games_from_input_file(input_file: String) -> Vec<Game> {
    let mut games: Vec<Game> = Vec::new();
    let mut game_reveals = Game::new();
    if let Ok(lines) = read_lines(input_file) {
        for line in lines.flatten() {
            let mut split_line = line.split_whitespace().collect::<Vec<&str>>();
            split_line.remove(0);
            split_line.remove(0);
            let split_line = split_line.join(" ");
            let all_reveals = split_line.split(';').collect::<Vec<&str>>();
            for reveal in all_reveals {
                let vectorized_reveal = reveal.split_whitespace().collect::<Vec<&str>>();
                game_reveals.add_reveal(get_game(vectorized_reveal));
            }
            games.push(game_reveals);
            game_reveals = Game::new();
        }
    }
    return games;
}
fn get_game(reveal: Vec<&str>) -> Reveal {
    let mut i = 0;
    let mut pair_of_die: Vec<(u32, String)> = Vec::new();
    while reveal.len() > i {
        pair_of_die.push((u32::from_str(reveal[i]).unwrap(), reveal[i + 1].to_string()));
        i += 2;
    }

    let mut to_return_reveal = Reveal::new();
    for pair in pair_of_die {
        match pair.1.as_ref() {
            "green" => {
                to_return_reveal.set_green(pair.0 as u32);
            }
            "green," => {
                to_return_reveal.set_green(pair.0 as u32);
            }
            "blue" => {
                to_return_reveal.set_blue(pair.0 as u32);
            }
            "blue," => {
                to_return_reveal.set_blue(pair.0 as u32);
            }
            "red" => {
                to_return_reveal.set_red(pair.0 as u32);
            }
            "red," => {
                to_return_reveal.set_red(pair.0 as u32);
            }
            _ => {}
        }
    }
    to_return_reveal
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
