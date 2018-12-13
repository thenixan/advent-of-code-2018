use std::io::BufRead;
use std::str::FromStr;

use days::print_header;
use days::read_file;

pub fn run_first_task() {
    print_header(7, 1);
    match read_file("days/7/INPUT")
        .map(|reader| first_task_job(reader)) {
        Ok(x) => println!("Result: {}", x),
        Err(_) => println!("Error"),
    };
}

fn first_task_job<T>(reader: T) -> String where T: BufRead {
    let route = read_to_route(reader);
    find_path(&route, "".to_string())
}

pub fn run_second_task() {
    print_header(7, 2);
    match read_file("days/7/INPUT")
        .map(|reader| second_task_job(reader, 61, 5)) {
        Ok(x) => println!("Result: {}", x),
        Err(_) => println!("Error"),
    }
}

fn second_task_job<T>(reader: T, task_length: u8, workers_count: u8) -> String where T: BufRead {
    let route = read_to_route(reader);
    let result = calc_time(&route, Environment::new(), Workers::new(workers_count), task_length);
    result.time.to_string()
}

const A_CHAR_NUMBER: u8 = 'A' as u8;

fn step_length(name: char, task_length: u8) -> u8 {
    let char_number = name as u8;
    char_number - A_CHAR_NUMBER + task_length
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

fn find_available(route: &Route, this: &String) -> Vec<char> {
    route.iter()
        .chain(vec!(&Connection::new(find_last(route), '_')))
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
        .collect()
}

fn find_path(route: &Route, this: String) -> String {
    match find_available(&route, &this).iter().min() {
        Some(x) => {
            let result = [this, x.to_string()].concat();
            find_path(&route, result)
        }
        None => this,
    }
}

fn calc_time(route: &Route, this: Environment, workers: Workers, step_time: u8) -> Environment {
    let events = workers.emit(this.time);

    let new_env = events.iter()
        .filter_map(|p| match p {
            Event::Finished(_) => Some(p),
            _ => None
        })
        .fold(this, |e: Environment, i: &Event| {
            match i {
                Event::Finished(name) => {
                    e.log_work(*name)
                }
                _ => e
            }
        });
    let new_workers = events.iter()
//        .filter(|p| match p {
//            Event::Idle => true,
//            _ => false
//        })
        .fold(workers, |w, _| {
            match find_available(route, &new_env.route)
                .iter()
                .filter(|p| !w.in_progress(new_env.time).contains(p))
                .min() {
                Some(x) => {
                    println!("Started: {}@{}", x, new_env.time);
                    w.start_job(new_env.time, *x, step_time)
                }
                None => {
                    w
                }
            }
        });
    println!("{:?} : {:?}", new_env, new_workers);
    if new_workers.is_all_idle(new_env.time) {
        new_env
    } else {
        calc_time(route, new_env.tick(), new_workers, step_time)
    }
}


fn read_to_route<T>(reader: T) -> Route where T: BufRead {
    reader.lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| line.parse::<Connection>().ok())
        .collect()
}

#[derive(Clone, Copy, Debug)]
enum Job {
    Empty,
    Active(i32, char),
}

impl Job {
    fn can_run_next(&self, now: i32) -> bool {
        match self {
            Job::Empty => true,
            Job::Active(time, _) => time <= &now,
        }
    }
}

#[derive(Clone, Debug)]
struct Workers {
    set: Vec<Job>,
}

enum Event {
    Idle,
    Finished(char),
}

impl Workers {
    fn new(number: u8) -> Workers {
        Workers { set: vec![Job::Empty; number as usize] }
    }

    fn start_job(&self, now: i32, name: char, step_time: u8) -> Workers {
        match self.set.iter()
            .position(|p| p.can_run_next(now)) {
            Some(x) => {
                let mut new_set = self.set
                    .iter()
                    .enumerate()
                    .filter(|p| p.0 != x)
                    .map(|p| p.1.clone())
                    .collect::<Vec<Job>>();
                new_set.push(Job::Active(now + step_length(name, step_time) as i32, name));
                Workers { set: new_set }
            }
            None => self.clone(),
        }
    }

    fn in_progress(&self, now: i32) -> Vec<char> {
        self.set.iter()
            .filter_map(|p| {
                match p {
                    Job::Empty => None,
                    Job::Active(time, name) => {
                        if time >= &now {
                            Some(*name)
                        } else {
                            None
                        }
                    }
                }
            })
            .collect()
    }

    fn is_all_idle(&self, now: i32) -> bool {
        self.set.iter().all(|p| match p {
            Job::Empty => true,
            Job::Active(time, _) => {
                time <= &now
            }
        })
    }

    fn emit(&self, now: i32) -> Vec<Event> {
        self.set
            .iter()
            .filter_map(|j| {
                match j {
                    Job::Empty => Some(Event::Idle),
                    Job::Active(time, name) => {
                        if time == &now {
                            Some(Event::Finished(*name))
                        } else if time < &now {
                            Some(Event::Idle)
                        } else {
                            None
                        }
                    }
                }
            })
            .collect()
    }
}

#[derive(Clone, Debug)]
struct Environment {
    route: String,
    time: i32,
}

impl Environment {
    fn new() -> Environment {
        Environment { route: "".to_string(), time: 0 }
    }

    fn tick(self) -> Environment {
        Environment { route: self.route, time: self.time + 1 }
    }

    fn log_work(self, name: char) -> Environment {
        Environment { route: [self.route, name.to_string()].concat(), time: self.time }
    }
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
    use days::seventh::second_task_job;

    const INPUT: &str = "Step C must be finished before step A can begin.\nStep C must be finished before step F can begin.\nStep A must be finished before step B can begin.\nStep A must be finished before step D can begin.\nStep B must be finished before step E can begin.\nStep D must be finished before step E can begin.\nStep F must be finished before step E can begin.";

    #[test]
    fn test_task_one() {
        assert_eq!("CABDFE".to_string(), first_task_job(INPUT.as_bytes()))
    }

    #[test]
    fn test_task_two() {
        assert_eq!("15".to_string(), second_task_job(INPUT.as_bytes(), 1, 2))
    }
}