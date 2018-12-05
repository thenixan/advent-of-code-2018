use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

pub fn run() {
    println!("Running day one");
    match File::open("days/1/input")
        .and_then(|f| {
            let reader = BufReader::new(&f);
            reader.lines().map(|s| {
                s.and_then(|s| {
                    s.parse::<i32>()
                })
            })
        }) {
        Ok(x) => println!("{}", x),
        Err(_) => unimplemented!(),
    };
}
