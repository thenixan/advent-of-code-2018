use std::io::BufRead;
use std::str::FromStr;

use days::print_header;
use days::read_file;

pub fn run_first_task() {
    print_header(7, 1);
    read_file("days/7/input");
}

fn first_task_job<T>(reader: T) -> String where T: BufRead {
    reader.lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| line.parse::<Vertex>())
        .scan(Option::None, |path, item| {
            match path {
//                None =>
            }
        });
    "".to_string()
}

#[derive(Clone)]
struct Link {
    next: Box<[Vertex]>
}


impl Link {
    fn new(v: &Vec<Vertex>) -> Link {
        Link { next: v.clone().into_boxed_slice() }
    }

    fn one(v: Vertex) -> Link {
        Link { next: [v].to_vec().into_boxed_slice() }
    }

    fn empty() -> Link {
        Link { next: [].to_vec().into_boxed_slice() }
    }

    fn is_last(&self) -> bool {
        self.next.is_empty()
    }

    fn add(&mut self, v: Vertex) -> &mut Self {
        let next = &mut self.next.to_vec();
        next.push(v);
        self
    }
}

impl From<Vertex> for Link {
    fn from(v: Vertex) -> Self {
        Link::new(&[v].to_vec())
    }
}

#[derive(Clone)]
struct Vertex {
    name: char,
    next: Link,
}

impl Vertex {
    fn last(name: char) -> Vertex {
        Vertex { name, next: Link::empty() }
    }

    fn new(name: char, next: Link) -> Vertex {
        Vertex { name, next }
    }
}

impl FromStr for Vertex {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let name = s.chars().collect::<Vec<_>>()[5];
        let target = s.chars().collect::<Vec<_>>()[36];
        Ok(Vertex::new(name, Link::one(Vertex::new(target, Link::empty()))))
    }
}