use std::collections::HashMap;
use std::io::BufRead;
use std::str::FromStr;

use days::print_header;
use days::read_file;

pub fn run_first_task() {
    print_header(9, 1);
    match read_file("days/9/input")
        .map(|reader| first_task_job(reader)) {
        Ok(x) => println!("Result: {}", x),
        Err(_) => println!("Error"),
    }
}

fn first_task_job<T>(reader: T) -> String where T: BufRead {
    match reader
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|l| l.parse::<Terms>().ok())
        .next() {
        Some(terms) => {
            (1..=terms.last_worth)
                .zip((1..=terms.players_number).cycle())
                .fold(Game::new(), |mut game, (term, player)| {
                    game.place_marble(player, term);
                    game
                })
                .max_score()
                .to_string()
        }
        None => "".to_string(),
    }
}

#[derive(Debug)]
struct Game {
    position: usize,
    set: Vec<i32>,
    players: HashMap<i32, i32>,
}

impl Game {
    fn new() -> Game {
        Game { position: 0, set: vec!(0), players: HashMap::new() }
    }

    fn max_score(&self) -> i32 {
        *self.players.iter().map(|(_, v)| v).max().unwrap()
    }

    fn move_to(&mut self, to: i32) {
        if !self.set.is_empty() {
            let new_position = self.position as i32 + to;
            if new_position < 0 {
                self.position = (self.set.len() as i32 + new_position) as usize;
            } else if new_position == 0 {
                self.position = self.set.len();
            } else if new_position <= self.set.len() as i32 {
                self.position = new_position as usize;
            } else {
                self.position = new_position as usize - self.set.len();
            }
        }
    }

    fn place_marble(&mut self, player: i32, number: i32) {
        if number % 23 == 0 {
            *self.players.entry(player).or_insert(0) += number;
            self.move_to(-7);
            let taken = self.set.remove(self.position);
            *self.players.entry(player).or_insert(0) += taken;
        } else {
            self.move_to(2);
            self.set.insert(self.position, number);
        }
    }
}

struct Terms {
    players_number: i32,
    last_worth: i32,
}

impl Terms {
    fn new(players_number: i32, last_worth: i32) -> Terms {
        Terms { players_number, last_worth }
    }
}

impl FromStr for Terms {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let input = s.split_whitespace().collect::<Vec<&str>>();
        let players_number = input.iter().nth(0).unwrap().parse::<i32>().unwrap();
        let last_worth = input.iter().nth(6).unwrap().parse::<i32>().unwrap();
        Ok(Terms::new(players_number, last_worth))
    }
}


#[cfg(test)]
mod tests {
    use days::ninth::first_task_job;

    const INPUT: &str = "10 players; last marble is worth 1618 points";

    #[test]
    fn test_task_one() {
        assert_eq!("8317".to_string(), first_task_job(INPUT.as_bytes()))
    }
}