use std::io::BufRead;
use std::str::FromStr;

use days::print_header;
use days::read_file;

pub fn run_first_task() {
    print_header(7, 1);
    read_file("days/7/input");
}

fn first_task_job<T>(reader: T) -> String where T: BufRead {
    let vertex_vec = &mut reader.lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| line.parse::<Connection>().ok())
        .collect::<Vec<_>>();

    let entry_point = find_entry_point(vertex_vec);

    let entry_vertex = Vertex::new(entry_point.from_name, Link::one(Vertex::last(entry_point.to_name)));
    fold_connections(entry_vertex, vertex_vec);

    "".to_string()
}

fn fold_connections(mut entry: Vertex, v: &mut Vec<Connection>) {
    if !v.is_empty() {
        match v.iter().position(|i| i.to_name == entry.name) {
            Some(x) => {
                let c = v.remove(x);
                entry.next.add(Vertex::last(c.to_name));
                fold_connections(entry, v);
            }
            None => {
                entry.next().into_iter().for_each(|item| fold_connections(item, v));
            }
        };
    };
}

fn find_entry_point(v: &mut Vec<Connection>) -> Connection {
    let targets = v.iter().map(|i| i.to_name).collect::<String>();
    let pos = v.iter_mut().position(|i| targets.contains(i.from_name)).unwrap();
    v.remove(pos)
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

    fn add(&mut self, v: Vertex) -> &mut Self {
        let n = &mut self.next.to_vec();
        n.push(v);
        self.next = n.clone().into_boxed_slice();
        self
    }

    fn is_last(&self) -> bool {
        self.next.is_empty()
    }
}

impl From<Vertex> for Link {
    fn from(v: Vertex) -> Self {
        Link::new(&[v].to_vec())
    }
}

#[derive(Clone)]
struct Connection {
    from_name: char,
    to_name: char,
}

impl Connection {
    fn new(from_name: char, to_name: char) -> Connection {
        Connection { from_name, to_name }
    }
}

impl FromStr for Connection {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let name = s.chars().collect::<Vec<_>>()[5];
        let target = s.chars().collect::<Vec<_>>()[36];
        Ok(Connection::new(name, target))
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

    fn next(self) -> Vec<Vertex> {
        self.next.next.to_vec()
    }
}

fn find(v: &Vertex, name: char) -> Option<Box<Vertex>> {
    if v.name == name {
        Some(Box(v))
    } else {
        let next = v.next();
        if next.is_empty() {
            None
        } else {
            match next.into_iter().find(|p| p.name == name) {
                Some(x) => Some(&x),
                None => next.into_iter().find_map(|item| find(&item, name)),
            }
        }
    }
}