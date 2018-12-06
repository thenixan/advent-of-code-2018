use std::env;

mod days;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.last() {
        Some(x) => match x.parse::<i32>() {
            Ok(x) => run_day(x),
            Err(_) => eprintln!("Day number must be a number"),
        }
        None => eprintln!("Provide day number")
    };
}

fn run_day(day: i32) {
    match day {
        1 => days::run_first_task(),
        2 => days::run_second_task(),
        _ => unimplemented!(),
    };
}
