use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

use days::print_header;
use days::read_file;

pub fn run_first_task() {
    print_header(6, 1);
    read_file("days/6/input")
        .map(|reader| first_task_job(reader));
}

fn first_task_job(reader: BufReader<File>) {
    let coordinates = reader.lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| line.parse::<Coordinate>().ok());

    let min_x = coordinates.min_by(|&l, &r| l.x.cmp(&r.x)).unwrap().x;
    let min_y = coordinates.min_by(|&l, &r| l.y.cmp(&r.y)).unwrap().y;
    let max_x = coordinates.max_by(|&l, &r| l.x.cmp(&r.x)).unwrap().x;
    let max_y = coordinates.max_by(|&l, &r| l.y.cmp(&r.y)).unwrap().y;

    let mut map = HashMap::new();
    for i in min_x..max_x + 1 {
        for j in min_y..max_y + 1 {
            match find_relation(i as i64, j as i64, coordinates) {
                Option::None => map.insert((i, j), PointRelation::Equal),
                Option::Some(c) => {
                    if i == min_x || i == max_x || j == min_y || j == max_y {
                        map.insert((i, j), PointRelation::Infinite(Box::new(c)));
                    } else {
                        let left = map.get(&(i - 1, j));
                        let top = map.get(&(i, j - 1));
                        let right = map.get(&(i + 1, j));
                        let bottom = map.get(&(i, j + 1));
                        if is_infinity(left) || is_infinity(right) || is_infinity(top) || is_infinity(bottom) {
                            map.insert((i, j), PointRelation::Infinite(Box::new(c)));
                        } else {
                            map.insert((i, j), PointRelation::Finite(Box::new(c)));
                        }
                    }
                }
            }
        }
    }
}

fn is_infinity(o: Option<&PointRelation>) -> bool {
    match o {
        Option::Some(&PointRelation::Infinite(_)) => true,
        _ => false,
    }
}

fn find_relation<T: Iterator<Item=Coordinate>>(x: i64, y: i64, coordinates: T) -> Option<Coordinate> {
    let (md, mc, o) = coordinates
        .map(|coordinate| (coordinate, coordinate.distance_to(x, y)))
        .fold((-1_i64, 0_i64, Option::None), |(min_distance, min_count, opt), (coordinate, distance)| {
            if min_distance == -1 {
                (distance, 1, Option::Some(coordinate))
            } else if min_distance > distance {
                (distance, 1, Option::Some(coordinate))
            } else if min_distance == distance {
                (distance, min_count + 1, Option::None)
            } else {
                (min_distance, min_count, opt)
            }
        });
    o
}

enum PointRelation {
    Infinite(Box<Coordinate>),
    Equal,
    Finite(Box<Coordinate>),
}

enum Error {
    ParseError
}

struct Coordinate {
    x: u32,
    y: u32,
}

impl Coordinate {
    fn new(x: u32, y: u32) -> Coordinate {
        Coordinate { x, y }
    }

    fn distance_to(&self, x: i64, y: i64) -> i64 {
        let cx = self.x as i64 - x.abs();
        let cy = self.y as i64 - y.abs();
        cx.abs() + cy.abs()
    }
}

impl FromStr for Coordinate {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let numbers: Vec<&str> = s.to_string().split(", ").collect();
        let parse_result_x = numbers[0].parse::<u32>();
        let parse_result_y = numbers[1].parse::<u32>();
        match (parse_result_x, parse_result_y) {
            (Ok(x), Ok(y)) => Ok(Coordinate::new(x, y)),
            _ => Err("cannot parse coordinates".to_string()),
        }
    }
}