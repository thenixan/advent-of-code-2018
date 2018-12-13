use std::io::BufRead;

use days::print_header;
use days::read_file;

pub fn run_first_task() {
    print_header(1, 1);
    match read_file("days/1/INPUT")
        .map(|reader| first_task_job(reader)) {
        Ok(x) => println!("Result is: {}", x),
        Err(_) => println!("Error"),
    };
}

fn first_task_job<T>(reader: T) -> i32 where T: BufRead {
    reader
        .lines()
        .filter_map(|res| res.ok())
        .map(|line| line.parse::<i32>())
        .filter_map(|res| res.ok())
        .sum::<i32>()
}

pub fn run_second_task() {
    print_header(1, 2);
    match read_file("days/1/INPUT")
        .map(|reader| second_task_job(reader)) {
        Ok(x) => println!("Result: {}", x),
        Err(_) => println!("Error"),
    };
}

fn second_task_job<T>(reader: T) -> i32 where T: BufRead {
    let frequencies = reader
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| line.parse::<i32>().ok())
        .collect::<Vec<_>>();

    operate_vec(&frequencies, &mut Vec::new(), 0, 1).unwrap()
}

fn operate_vec(v: &Vec<i32>, r: &mut Vec<i32>, sum: i32, i: i32) -> Result<i32, String> {
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


#[cfg(test)]
mod tests {
    use test::Bencher;

    use days::first;

    #[bench]
    fn bench_first(b: &mut Bencher) {
        b.iter(|| first::run_first_task());
    }

    #[bench]
    fn bench_second(b: &mut Bencher) {
        b.iter(|| first::run_second_task());
    }
}