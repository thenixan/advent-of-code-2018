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

fn run_day(_day: i32) {
    days::run();
}
