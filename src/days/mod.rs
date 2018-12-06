use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub mod first;
pub mod second;
pub mod third;

pub fn print_header(day: i32, task: i32) {
    println!("Running day {}, task {}", day, task);
}

pub fn read_file_to_vec<T, F>(path: &str, f: F) -> Result<Vec<T>, String> where F: Fn(String) -> Result<T, String> {
    File::open(path)
        .map_err(|e| e.to_string())
        .map(|file| BufReader::new(file))
        .map(|r| r.lines())
        .map(|l|
            l.flat_map(move |line| {
                line.map_err(|e| e.to_string())
                    .map(|s| f(s))
                    .and_then(|r| r)
            }))
        .map(|v| v.collect::<Vec<_>>())
}