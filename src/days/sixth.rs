use std::collections::HashMap;
use std::io::BufRead;
use std::str::FromStr;

use days::print_header;
use days::read_file;

pub fn run_first_task() {
    print_header(6, 1);
    match read_file("days/6/input")
        .map(|reader| first_task_job(reader)) {
        Ok(x) => println!("Result: {}", x),
        Err(_) => println!("Error"),
    };
}

fn first_task_job<T>(reader: T) -> i32 where T: BufRead {
    let coordinates = reader
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| line.parse::<Coordinate>().ok())
        .collect::<Vec<_>>();

    let result = counts(&coordinates);

    *result.values().filter(|&&v| v != -1).max().unwrap()
}

fn counts(coordinates: &Vec<Coordinate>) -> HashMap<Coordinate, i32> {
    let bounds = &Bounds::new(&coordinates);

    (bounds.min_x..=bounds.max_x)
        .flat_map(move |x| {
            (bounds.min_y..=bounds.max_y).map(move |y| Coordinate::new(x, y))
        })
        .fold(HashMap::new(), |mut map, coordinate| {
            let relation = find_relation(coordinate, &coordinates);
            match relation {
                Option::None => {
                    map.insert(coordinate, PointRelation::Equal);
                }
                Option::Some(c) => {
                    if bounds.is_bounded(coordinate) {
                        map.insert(coordinate, PointRelation::Infinite(c));
                    } else if check_infinity(&map, coordinate, c) {
                        map.insert(coordinate, PointRelation::Infinite(c));
                    } else {
                        map.insert(coordinate, PointRelation::Finite(c));
                    }
                }
            };
            map
        })
        .clone()
        .values()
        .fold(HashMap::new(), |mut counts, &v| {
            match v {
                PointRelation::Equal => {}
                PointRelation::Finite(c) => {
                    let x = counts.entry(c).or_insert(0);
                    if *x != -1 {
                        *x += 1;
                    }
                }
                PointRelation::Infinite(c) => {
                    *counts.entry(c).or_insert(-1) = -1;
                }
            }
            counts
        })
}

fn check_infinity(map: &HashMap<Coordinate, PointRelation>, coordinate: Coordinate, relation: Coordinate) -> bool {
    let left = map.get(&coordinate.left());
    let right = map.get(&coordinate.right());
    let top = map.get(&coordinate.top());
    let bottom = map.get(&coordinate.bottom());
    if is_infinity(left, relation) || is_infinity(right, relation) || is_infinity(top, relation) || is_infinity(bottom, relation) {
        true
    } else {
        false
    }
}

fn is_infinity(o: Option<&PointRelation>, relation: Coordinate) -> bool {
    match o {
        Option::Some(PointRelation::Infinite(c)) => c == &relation,
        _ => false,
    }
}

struct Bounds {
    min_x: u32,
    max_x: u32,
    min_y: u32,
    max_y: u32,
}

impl Bounds {
    fn new(coordinates: &Vec<Coordinate>) -> Bounds {
        let min_x = coordinates.iter().min_by(|&l, &r| l.x.cmp(&r.x)).unwrap().x;
        let min_y = coordinates.iter().min_by(|&l, &r| l.y.cmp(&r.y)).unwrap().y;
        let max_x = coordinates.iter().max_by(|&l, &r| l.x.cmp(&r.x)).unwrap().x;
        let max_y = coordinates.iter().max_by(|&l, &r| l.y.cmp(&r.y)).unwrap().y;
        Bounds { min_x, max_x, min_y, max_y }
    }

    fn is_bounded(&self, coordinate: Coordinate) -> bool {
        coordinate.x == self.min_x || coordinate.x == self.max_x || coordinate.y == self.min_y || coordinate.y == self.max_y
    }
}

fn find_relation(coordinate: Coordinate, coordinates: &Vec<Coordinate>) -> Option<Coordinate> {
    let (_, _, o) = coordinates
        .into_iter()
        .fold((-1_i64, 0_i64, Option::None), |(min_distance, min_count, opt), c| {
            let distance = c.distance_to(&coordinate);
            if min_distance == -1 {
                (distance, 1, Option::Some(*c))
            } else if min_distance > distance {
                (distance, 1, Option::Some(*c))
            } else if min_distance == distance {
                (distance, min_count + 1, Option::None)
            } else {
                (min_distance, min_count, opt)
            }
        });
    o
}

#[derive(Hash, Copy, Clone)]
enum PointRelation {
    Infinite(Coordinate),
    Equal,
    Finite(Coordinate),
}

#[derive(Hash, Copy, Clone)]
struct Coordinate {
    x: u32,
    y: u32,
}

impl Coordinate {
    fn new(x: u32, y: u32) -> Coordinate {
        Coordinate { x, y }
    }

    fn distance_to(&self, coordinate: &Coordinate) -> i64 {
        let cx = self.x as i64 - (coordinate.x as i64).abs();
        let cy = self.y as i64 - (coordinate.y as i64).abs();
        cx.abs() + cy.abs()
    }

    fn left(&self) -> Coordinate {
        Coordinate { x: self.x - 1, y: self.y }
    }

    fn right(&self) -> Coordinate {
        Coordinate { x: self.x + 1, y: self.y }
    }

    fn top(&self) -> Coordinate {
        Coordinate { x: self.x, y: self.y - 1 }
    }

    fn bottom(&self) -> Coordinate {
        Coordinate { x: self.x, y: self.y + 1 }
    }
}

impl std::cmp::Eq for Coordinate {}

impl std::cmp::PartialEq for Coordinate {
    fn eq(&self, r: &Self) -> bool {
        self.x.eq(&r.x) && self.y.eq(&r.y)
    }
}

impl FromStr for Coordinate {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let numbers = s.split(", ").collect::<Vec<_>>();
        let parse_result_x = numbers[0].parse::<u32>();
        let parse_result_y = numbers[1].parse::<u32>();
        match (parse_result_x, parse_result_y) {
            (Ok(x), Ok(y)) => Ok(Coordinate::new(x, y)),
            _ => Err("cannot parse coordinates".to_string()),
        }
    }
}


#[cfg(test)]
mod tests {
    use days::sixth::first_task_job;

    #[test]
    fn test_task_one() {
        assert_eq!(17, first_task_job("1, 1\n1, 6\n8, 3\n3, 4\n5, 5\n8, 9".as_bytes()))
    }
}