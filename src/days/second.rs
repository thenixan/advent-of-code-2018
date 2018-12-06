use days::print_header;
use days::read_file_to_vec;

pub fn run_first_task() {
    print_header(2, 1);
    match read_file_to_vec("days/2/input", |s| Ok(String::from(s)))
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

struct LineResult {
    has_doubles: bool,
    has_triples: bool,
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