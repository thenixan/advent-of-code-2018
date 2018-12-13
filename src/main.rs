#![feature(test)]
#![feature(stdsimd)]
extern crate chrono;
extern crate core;
extern crate test;
extern crate unic_char_range;

use std::error;
use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use std::io::Write;

mod days;

type Result<T> = std::result::Result<T, InputError>;

#[derive(Debug)]
enum InputError {
    ParseError(std::num::ParseIntError),
    StreamError(std::io::Error),
}

impl From<std::num::ParseIntError> for InputError {
    fn from(e: std::num::ParseIntError) -> Self {
        InputError::ParseError(e)
    }
}

impl From<std::io::Error> for InputError {
    fn from(e: std::io::Error) -> Self {
        InputError::StreamError(e)
    }
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            InputError::ParseError(_) => write!(f, "ParseError"),
            InputError::StreamError(_) => write!(f, "StreamError"),
        }
    }
}

impl error::Error for InputError {
    fn description(&self) -> &str {
        match self {
            InputError::ParseError(e) => e.description(),
            InputError::StreamError(e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            InputError::ParseError(e) => e.cause(),
            InputError::StreamError(e) => e.cause(),
        }
    }
}

fn main() {
    let mut day = 0;
    while day == 0 {
        match read_input("Enter day number") {
            Ok(x) => day = x,
            Err(e) => println!("Wrong day number format: {}", e.description()),
        }
    }

    let mut task = 0;
    while task == 0 {
        match read_input("Enter task number") {
            Ok(x) => task = x,
            Err(e) => println!("Wrong task number format: {}", e.description()),
        }
    }

    run_day((day, task));
}

fn read_input(prompt: &str) -> Result<i32> {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    match write!(&mut stdout, "{}: ", prompt) {
        Ok(_) => {
            stdout.flush()
                .map_err(|e| InputError::StreamError(e))
                .map(|()| {
                    let mut input = String::new();
                    stdin.read_line(&mut input)
                        .map_err(|e| InputError::from(e))
                        .and_then(|_| {
                            input
                                .trim()
                                .parse::<i32>()
                                .map_err(|e| InputError::from(e))
                        })
                }).and_then(|s| s)
        }
        Err(e) => Err(InputError::StreamError(e)),
    }
}

fn run_day(task: (i32, i32)) {
    match task {
        (1, 1) => days::first::run_first_task(),
        (1, 2) => days::first::run_second_task(),
        (2, 1) => days::second::run_first_task(),
        (2, 2) => days::second::run_second_task(),
        (3, 1) => days::third::run_first_task(),
        (3, 2) => days::third::run_second_task(),
        (4, 1) => days::fourth::run_first_task(),
        (4, 2) => days::fourth::run_second_task(),
        (5, 1) => days::fifth::run_first_task(),
        (5, 2) => days::fifth::run_second_task(),
        (6, 1) => days::sixth::run_first_task(),
        (6, 2) => days::sixth::run_second_task(),
        (7, 1) => days::seventh::run_first_task(),
        (7, 2) => days::seventh::run_second_task(),
        (8, 1) => days::eighth::run_first_task(),
        (8, 2) => days::eighth::run_second_task(),
        (9, 1) => days::ninth::run_first_task(),
        (9, 2) => days::ninth::run_second_task(),
        _ => println!("Day and task is not defined!"),
    }
}
