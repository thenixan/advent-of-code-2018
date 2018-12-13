use days::print_header;
use days::read_file_to_vec;

pub fn run_first_task() {
    print_header(2, 1);
    match read_file_to_vec("days/2/INPUT", |s| Ok(s))
        .map(|v| {
            v.into_iter().map(|s| LineResult::from_line(s)).fold((0, 0), |counter, s| {
                match s {
                    LineResult { has_doubles: true, has_triples: true } => (counter.0 + 1, counter.1 + 1),
                    LineResult { has_doubles: true, has_triples: false } => (counter.0 + 1, counter.1),
                    LineResult { has_doubles: false, has_triples: true } => (counter.0, counter.1 + 1),
                    _ => counter
                }
            })
        }) {
        Ok((doubles, triples)) => {
            let result = doubles * triples;
            println!("Result: {}", result);
        }
        Err(e) => println!("{}", e),
    };
}

pub fn run_second_task() {
    print_header(2, 2);
    match read_file_to_vec("days/2/INPUT", |s| Ok(s))
        .map(|vec| {
            let mut i = 0;
            let mut result = Option::None;
            while i < vec.len() - 1 && result == Option::None {
                result = compare_vec(&vec, i);
                i += 1;
            }
            result
        }) {
        Ok(x) => match x {
            Some(x) => println!("Result: {}", x),
            None => println!("Found nothing"),
        },
        Err(e) => println!("{}", e),
    };
}

struct LineResult {
    has_doubles: bool,
    has_triples: bool,
}

fn compare_vec(v: &Vec<String>, start: usize) -> Option<String> {
    let s = &v[start];
    let mut r = Option::None;
    let mut j = start + 1;
    while j < v.len() && r == Option::None {
        r = compare_strings(s, &v[j]);
        j += 1;
    }
    r
}

fn compare_strings(first: &str, second: &str) -> Option<String> {
    let mut non_valid = Option::None;
    let mut i = 0;
    let mut non_valid_count = 0;
    while non_valid_count < 2 && i < first.len() {
        let f = &first[i..i + 1];
        let s = &second[i..i + 1];
        if f != s {
            non_valid = Some(i);
            non_valid_count += 1;
        }
        i += 1;
    }
    if non_valid_count > 1 {
        non_valid = Option::None;
    }
    match non_valid {
        None => None,
        Some(x) => Some([first[..x].to_string(), first[x + 1..].to_string()].concat())
    }
}

impl LineResult {
    fn new(has_doubles: bool, has_triples: bool) -> LineResult {
        LineResult { has_doubles, has_triples }
    }

    fn from_line(s: String) -> LineResult {
        s.chars().into_iter().fold(LineResult::new(false, false), |i, c| {
            match s.chars().filter(|o| *o == c).count() {
                3 => LineResult::new(i.has_doubles, true),
                2 => LineResult::new(true, i.has_triples),
                _ => i
            }
        })
    }
}