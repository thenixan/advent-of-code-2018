use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;

use days::print_header;
use days::read_file_to_vec;

pub fn run_first_task() {
    print_header(3, 1);
    match read_file_to_vec("days/3/input", |s| {
        s.parse::<Tile>()
    }).map(|tiles| {
        tiles.into_iter().fold(&mut HashMap::new(), |plot, item| {
            for i in item.padding_left..item.padding_left + item.size.width {
                for j in item.padding_top..item.padding_top + item.size.height {
                    let val = plot.entry((i, j)).or_insert_with(|| 0);
                    *val += 1;
                }
            }
            plot
        }).to_owned()
    }).map(|plot| {
        plot.into_iter().filter(|(_, v)| *v > 1).count()
    }) {
        Ok(x) => println!("Result: {}", x),
        Err(e) => println!("{}", e),
    };
}


struct Size {
    height: i32,
    width: i32,
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