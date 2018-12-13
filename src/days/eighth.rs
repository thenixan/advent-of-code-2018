use std::io::BufRead;

use days::print_header;
use days::read_file;

pub fn run_first_task() {
    print_header(8, 1);
    match read_file("days/8/input")
        .map(|reader| first_task_job(reader)) {
        Ok(x) => println!("Result: {}", x),
        Err(_) => println!("Error"),
    }
}

fn first_task_job<T>(mut reader: T) -> String where T: BufRead {
    let result = read(&mut reader);
    result.sum_meta().to_string()
}

pub fn run_second_task() {
    print_header(8, 1);
    match read_file("days/8/input")
        .map(|reader| second_task_job(reader)) {
        Ok(x) => println!("Result: {}", x),
        Err(_) => println!("Error"),
    }
}

fn second_task_job<T>(mut reader: T) -> String where T: BufRead {
    let result = read(&mut reader);
    result.sum_value().to_string()
}

fn read<T>(reader: &mut T) -> Node where T: BufRead {
    let child_count = read_child_count(reader);
    let metadata_count = read_metadata_count(reader);
    let childs = (0..child_count)
        .fold(vec!(), |mut v, _| {
            v.push(Box::new(read(reader)));
            v
        });
    let metadata = read_metadata(reader, metadata_count);
    Node::new(childs, metadata)
}

fn read_int<T>(reader: &mut T) -> i32 where T: BufRead {
    let mut buf = vec!();
    reader.read_until(b' ', &mut buf).unwrap();
    std::str::from_utf8(&buf).unwrap().trim().parse::<i32>().unwrap()
}

fn read_child_count<T>(reader: &mut T) -> i32 where T: BufRead {
    read_int(reader)
}

fn read_metadata_count<T>(reader: &mut T) -> i32 where T: BufRead {
    read_int(reader)
}

fn read_metadata<T>(reader: &mut T, count: i32) -> Vec<i32> where T: BufRead {
    (0..count)
        .fold(vec!(), |mut v, _| {
            v.push(read_int(reader));
            v
        })
}

struct Node {
    next: Vec<Box<Node>>,
    metadata: Vec<i32>,
}

impl Node {
    fn new(next: Vec<Box<Node>>, metadata: Vec<i32>) -> Node {
        Node { next, metadata }
    }

    fn sum_meta(&self) -> i32 {
        self.next.iter().map(|n| n.sum_meta()).sum::<i32>() + self.metadata.iter().sum::<i32>()
    }

    fn sum_value(&self) -> i32 {
        if self.next.is_empty() {
            self.metadata.iter().sum()
        } else {
            self.metadata.iter()
                .fold(0, |s, v| {
                    match self.next.iter().nth(*v as usize - 1) {
                        Some(x) => s + x.sum_value(),
                        None => s,
                    }
                })
        }
    }
}


#[cfg(test)]
mod tests {
    use days::eighth::first_task_job;
    use days::eighth::second_task_job;

    const INPUT: &str = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

    #[test]
    fn test_task_one() {
        assert_eq!("138".to_string(), first_task_job(INPUT.as_bytes()))
    }

    #[test]
    fn test_task_two() {
        assert_eq!("66".to_string(), second_task_job(INPUT.as_bytes()))
    }
}