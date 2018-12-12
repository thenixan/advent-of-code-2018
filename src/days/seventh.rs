use std::io::BufRead;
use std::str::FromStr;

use days::print_header;
use days::read_file;

pub fn run_first_task() {
    print_header(7, 1);
    match read_file("days/7/input").map(|reader| first_task_job(reader)) {
        Ok(x) => println!("Result: {}", x),
        Err(_) => println!("Error"),
    };
}

fn first_task_job<T>(reader: T) -> String where T: BufRead {
    let route = read_to_route(reader);
    let result = find_path_v2(&route, "".to_string());
    [result, find_last(&route).to_string()].concat()

//    let vertex_vec = &mut reader.lines()
//        .filter_map(|line| line.ok())
//        .filter_map(|line| line.parse::<Connection>().ok())
//        .collect::<Route>();
//
//    let entry_point = find_entry_point(vertex_vec);
//
//    let mut entry_vertex = Vertex::new(entry_point.from_name, Link::one(Vertex::last(entry_point.to_name)));
//    fold_connections(&mut entry_vertex, vertex_vec);
//
//
//    println!("{:?}", entry_vertex);
//
//    entry_vertex.fold_to(1).into_iter().collect()
}


type Route = Vec<Connection>;

fn find_last(route: &Route) -> char {
    route
        .iter()
        .fold("".to_string(), |mut s, i| {
            s.push(i.to_name);
            s.replace(i.from_name, "")
        })
        .chars().nth(0).unwrap()
}

fn find_prev(route: &Route, this: char) -> Step {
    route
        .iter()
        .filter(|c| c.to_name == this)
        .map(|c| c.from_name)
        .collect()
}

fn find_available(route: &Route, this: &String) -> Option<char> {
    route.iter()
        .filter(|c| !this.contains(&c.from_name.to_string()))
        .filter(|c| {
            find_prev(&route, c.from_name)
                .iter()
                .all(|f| {
                    this.contains(&f.to_string())
                })
        })
        .map(|c| {
            c.from_name
        })
        .min()
}

fn find_path_v2(route: &Route, this: String) -> String {
    match find_available(&route, &this) {
        Some(x) => {
            let result = [this, x.to_string()].concat();
            find_path_v2(&route, result)
        }
        None => this,
    }
}


fn read_to_route<T>(reader: T) -> Route where T: BufRead {
    reader.lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| line.parse::<Connection>().ok())
        .collect()
}

#[derive(Clone, Debug)]
struct Position {
    next_step: Step,
    prev_step: Step,
}

type Step = Vec<char>;

#[derive(Clone, Debug)]
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


#[cfg(test)]
mod tests {
    use days::seventh::first_task_job;

    const input: &str = "Step C must be finished before step A can begin.\nStep C must be finished before step F can begin.\nStep A must be finished before step B can begin.\nStep A must be finished before step D can begin.\nStep B must be finished before step E can begin.\nStep D must be finished before step E can begin.\nStep F must be finished before step E can begin.";

    #[test]
    fn test_task_one() {
        assert_eq!("CABDFE".to_string(), first_task_job(input.as_bytes()))
    }
}