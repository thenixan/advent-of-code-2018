use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use std::io::BufRead;
use std::str::FromStr;

use days::print_header;
use days::read_file;

pub fn run_first_task() {
    print_header(3, 1);
    match read_file("days/3/INPUT")
        .map(|reader| first_task_job(reader)) {
        Ok(x) => println!("Result: {}", x),
        Err(_) => println!("Error"),
    };
}

fn first_task_job<T>(reader: T) -> usize where T: BufRead {
    reader
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| line.parse::<Tile>().ok())
        .fold(HashMap::new(), |mut plot, tile| {
            for i in tile.padding_left..tile.padding_left + tile.size.width {
                for j in tile.padding_top..tile.padding_top + tile.size.height {
                    let val = plot.entry((i, j)).or_insert_with(|| 0);
                    *val += 1;
                }
            }
            plot
        })
        .iter()
        .filter(|&(_, v)| { *v > 1 })
        .count()
}

pub fn run_second_task() {
    print_header(3, 2);
    match read_file("days/3/INPUT")
        .map(|reader| second_task_job(reader)) {
        Ok(x) => println!("Result: {}", x),
        Err(_) => println!("Error")
    };
}

fn second_task_job<T>(reader: T) -> String where T: BufRead {
    let tiles = reader
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| line.parse::<Tile>().ok())
        .collect::<Vec<_>>();

    let plot = tiles
        .iter()
        .fold(HashMap::new(), |mut plot, tile| {
            for i in tile.padding_left..tile.padding_left + tile.size.width {
                for j in tile.padding_top..tile.padding_top + tile.size.height {
                    let val = plot.entry((i, j)).or_insert_with(|| 0);
                    *val += 1;
                }
            }
            plot
        })
        .into_iter()
        .filter(|&(_, v)| { v == 1 })
        .collect::<HashMap<_, _>>();

    tiles.into_iter().find(|tile| tile.find_disconnected(&plot).is_some()).unwrap().id
}

type Plot = HashMap<(i32, i32), i32>;

struct Size {
    height: i32,
    width: i32,
}

impl Tile {
    fn find_disconnected(&self, plot: &Plot) -> Option<&Tile> {
        let mut i = self.padding_left;
        let mut result = true;
        while result && i < self.padding_left + self.size.width {
            let mut j = self.padding_top;
            while result && j < self.padding_top + self.size.height {
                match plot.get(&(i, j)) {
                    Some(x) => if *x != 1 { result = false; },
                    None => result = false,
                }
                j += 1
            }
            i += 1;
        }
        match result {
            true => Some(self),
            false => None,
        }
    }
}

impl Size {
    fn new(height: i32, width: i32) -> Size {
        Size { height, width }
    }
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "(h: {}, w: {}", self.height, self.width)
    }
}

impl FromStr for Size {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let parts: Vec<&str> = s.split(|c| c == 'x').collect();
        let width = parts[0].parse::<i32>();
        let height = parts[1].parse::<i32>();
        match (width, height) {
            (Ok(w), Ok(h)) => Ok(Size::new(h, w)),
            _ => Err("error parsing sizes".to_string()),
        }
    }
}

struct Tile {
    id: String,
    size: Size,
    padding_top: i32,
    padding_left: i32,
}

impl Tile {
    fn new(id: String, padding_left: i32, padding_top: i32, size: Size) -> Tile {
        Tile { id, size, padding_top, padding_left }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "id: {}, size: {}, left: {}, top: {}", self.id, self.size, self.padding_left, self.padding_top)
    }
}

impl FromStr for Tile {
    type Err = String;

    fn from_str(s: &str) -> Result<Tile, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        let id: &str = &parts[0][1..];
        let paddings: Vec<&str> = parts[2].split(|c| c == ',' || c == ':').collect();
        let padding_left = paddings[0].parse::<i32>();
        let padding_top = paddings[1].parse::<i32>();
        match (padding_left, padding_top) {
            (Ok(left), Ok(top)) => {
                let size = parts[3].parse::<Size>();
                match size {
                    Ok(s) => Ok(Tile::new(id.to_string(), left, top, s)),
                    Err(e) => Err(e)
                }
            }
            _ => Err("cannot parse paddings".to_string()),
        }
    }
}


#[cfg(test)]
mod tests {
    use test::Bencher;

    use days::third::first_task_job;
    use days::third::second_task_job;

    #[test]
    fn test_task_one() {
        assert_eq!(4, first_task_job("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2".as_bytes()))
    }

    #[bench]
    fn bench_first(b: &mut Bencher) {
        b.iter(|| first_task_job("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2".as_bytes()));
    }

    #[test]
    fn test_task_two() {
        assert_eq!("3", second_task_job("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2".as_bytes()))
    }

    #[bench]
    fn bench_second(b: &mut Bencher) {
        b.iter(|| second_task_job("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2".as_bytes()));
    }
}