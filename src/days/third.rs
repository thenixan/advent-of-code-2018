use std::num::ParseIntError;
use std::str::FromStr;

use days::print_header;
use days::read_file_to_vec;

pub fn run_first_task() {
    print_header(3, 1);
    let a = read_file_to_vec("days/3/input", |s| {
        s.parse::<Tile>()
    });
}

struct Size {
    height: i32,
    width: i32,
}

impl Size {
    fn new(height: i32, width: i32) -> Size {
        Size { height, width }
    }

    fn square(&self) -> i32 {
        self.width * self.height
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

impl FromStr for Tile {
    type Err = String;

    fn from_str(s: &str) -> Result<Tile, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<_>>();
        let id = parts[0].chars().skip(1).collect::<&str>();
        let paddings = parts[2].split(',');
        unimplemented!()
    }
}