use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

pub mod first;
pub mod second;
pub mod third;
pub mod fourth;
pub mod fifth;
pub mod sixth;
pub mod seventh;
pub mod eighth;

pub fn print_header(day: i32, task: i32) {
    println!("Running day {}, task {}", day, task);
}

pub fn read_file_to_vec<T, F>(path: &str, f: F) -> Result<Vec<T>, String> where F: Fn(String) -> Result<T, String> {
    read_file(path)
        .map_err(|e| e.to_string())
        .map(|reader| reader.lines())
        .map(|lines| {
            lines.flat_map(move |line| {
                line.map_err(|e| e.to_string())
                    .map(|s| f(s))
                    .and_then(|r| r)
            })
        })
        .map(|v| v.collect::<Vec<_>>())
}

pub fn read_file(path: &str) -> Result<BufReader<File>, io::Error> {
    File::open(path).map(|file| BufReader::new(file))
}