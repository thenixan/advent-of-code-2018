use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn run() {
    println!("Running day one");
    let v = File::open("days/1/input")
        .map_err(|e| e.to_string())
        .map(|f: File| {
            BufReader::new(&f).lines().flat_map(|l| {
                l.
                    map_err(|e| e.to_string())
                    .map(|s| {
                        s.parse::<i32>()
                            .map_err(|e| e.to_string())
                    })
                    .and_then(|r| r)
            }).collect::<Vec<_>>()
        }).map(|v| v.iter().sum::<i32>());
    match v {
        Ok(x) => println!("{}", x),
        Err(_) => unimplemented!(),
    }
}
