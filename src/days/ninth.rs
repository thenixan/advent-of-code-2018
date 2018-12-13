use std::io::BufRead;
use std::str::FromStr;

use chrono::Local;

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
//                .filter(|(term, _)| term % 23 == 0)
                .fold(Game::new(terms.last_worth, terms.players_number), |mut game, (term, player)| {
                    game.place_marble(player, term);
                    game
                })
                .max_score()
                .to_string()
        }
        None => "".to_string(),
    }
}

pub fn run_second_task() {
    print_header(9, 2);
    println!("Started: {}", Local::now());
    match read_file("days/9/input")
        .map(|reader| second_task_job(reader)) {
        Ok(x) => println!("Result: {}", x),
        Err(_) => println!("Error"),
    }
    println!("Started: {}", Local::now());
}

fn second_task_job<T>(reader: T) -> String where T: BufRead {
    match reader
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|l| l.parse::<Terms>().ok())
        .next() {
        Some(terms) => {
            (1..=(terms.last_worth * 100))
                .zip((1..=terms.players_number).cycle())
                .fold(Game::new(terms.last_worth * 100, terms.players_number), |mut game, (term, player)| {
                    if term % 250_000 == 0 {
                        println!("Started: {} - {}", Local::now(), term);
                    }
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
    set: Vec<usize>,
    players: Vec<usize>,
}

impl Game {
    fn new(rounds_capacity: usize, players_capacity: usize) -> Game {
        let mut set: Vec<usize> = Vec::with_capacity(rounds_capacity);
        set.push(0);
        Game { position: 0, set, players: vec![0; players_capacity] }
    }

    fn max_score(&self) -> usize {
        *self.players.iter().max().unwrap()
    }

    fn move_to(&mut self, to: i32) {
        let new_position = (((2 * self.set.len()) as i32 + self.position as i32 + to) % self.set.len() as i32) as usize;
        if new_position == 0 {
            self.position = self.set.len();
        } else {
            self.position = new_position;
        }
    }

    fn place_marble(&mut self, player: usize, number: usize) {
        if number % 23 == 0 {
            self.move_to(-7);
            let taken = self.set.remove(self.position);
            self.players[player - 1] += number + taken;
        } else {
            self.move_to(2);
            self.set.insert(self.position, number);
        }
    }
}

struct Terms {
    players_number: usize,
    last_worth: usize,
}

impl Terms {
    fn new(players_number: usize, last_worth: usize) -> Terms {
        Terms { players_number, last_worth }
    }
}

impl FromStr for Terms {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let input = s.split_whitespace().collect::<Vec<&str>>();
        let players_number = input.iter().nth(0).unwrap().parse::<usize>().unwrap();
        let last_worth = input.iter().nth(6).unwrap().parse::<usize>().unwrap();
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