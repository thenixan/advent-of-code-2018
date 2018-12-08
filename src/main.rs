extern crate chrono;
extern crate core;

use core::borrow::BorrowMut;
use std::error;
use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use std::io::Stdout;
use std::io::Write;
use std::num::ParseIntError;

mod days;

type Result<T> = std::result::Result<T, InputError>;

#[derive(Debug)]
enum InputError {
    ParseError(std::num::ParseIntError),
    StreamError(std::io::Error),
}

impl From<ParseIntError> for InputError {
    fn from(e: ParseIntError) -> InputError {
        InputError::ParseError(e)
    }
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            InputError::ParseError(_) => write!(f, "ParseError"),
            InputError::FlushError(_) => write!(f, "FlushError"),
            InputError::ReadStdInError(_) => write!(f, "ReadStdInError"),
        }
    }
}

impl error::Error for InputError {
    fn description(&self) -> &str {
        match self {
            InputError::ParseError(e) => e.description(),
            InputError::FlushError(e) => e.description(),
            InputError::ReadStdInError(e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            InputError::ParseError(e) => e.cause(),
            InputError::FlushError(e) => e.cause(),
            InputError::ReadStdInError(e) => e.cause()
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

    write!(&mut stdout, "{}: ", prompt);
    stdout.flush()
        .map_err(|e| InputError::FlushError(e))
        .map(|()| {
            let mut input = String::new();
            stdin.read_line(&mut input)
                .map_err(|e| InputError::ReadStdInError(e))
                .and_then(|_s| input.trim().parse::<i32>().map_err(|e| InputError::ParseError(e)))
        }).and_then(|s| s)
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
        _ => println!("Day and task is not defined!"),
    }
}
