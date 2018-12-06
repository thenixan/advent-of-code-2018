use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn run_first_task() {
    print_header(1, 1);
    match read_file_to_vec("days/1-1/input")
        .map(|v| v.iter().sum::<i32>()) {
        Ok(x) => println!("Result is: {}", x),
        Err(_e) => println!("{}", _e),
    };
}

pub fn run_second_task() {
    print_header(1, 2);
    let mut results = Vec::new();
    match read_file_to_vec("days/1-2/input").and_then(|v| {
        operate_vec(&v, &mut results, 0, 1)
    }) {
        Ok(x) => println!("First occurance: {}", x),
        Err(_e) => println!("{}", _e),
    };
}

fn operate_vec(v: &Vec<i32>, r: &mut Vec<i32>, sum: i32, i: i32) -> Result<i32, String> {
    println!("Running {} iteration", i);
    let mut s = sum;
    match v.iter().find(|item| {
        s = s + **item;
        match r.binary_search(&s) {
            Ok(_x) => true,
            Err(_i) => {
                r.insert(_i, s);
                false
            }
        }
    }) {
        Some(_x) => Ok(s),
        None => operate_vec(v, r, s, i + 1),
    }
}

fn print_header(day: i32, task: i32) {
    println!("Running day {}, task {}", day, task);
}

fn read_file_to_vec(path: &str) -> Result<Vec<i32>, String> {
    File::open(path)
        .map_err(|e| e.to_string())
        .map(|file| BufReader::new(file))
        .map(|r| r.lines())
        .map(|l| l.flat_map(|line| {
            line.map_err(|e| e.to_string())
                .map(|s| s.parse::<i32>().map_err(|e| e.to_string()))
                .and_then(|r| r)
        }))
        .map(|v| v.collect::<Vec<_>>())
}