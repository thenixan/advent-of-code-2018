use std::str::FromStr;

use chrono;
use chrono::DateTime;
use chrono::offset::TimeZone;
use chrono::Utc;

use days::print_header;
use days::read_file_to_vec;

pub fn run_first_task() {
    print_header(4, 1);
    read_file_to_vec("days/4/input", |s| s.parse::<Record>());
}

struct Record {
    timestamp: DateTime<Utc>,
    event: Event,
}

impl FromStr for Record {
    type Err = String;

    fn from_str(s: &str) -> Result<Record, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<_>>();
        let timestamp = Utc.datetime_from_str(&[parts[0], parts[1]].join(" "), "[%Y-%m-%d %H:%M]");
        let a = parts[2..].join(" ");
        println!("{}", a);
        Err("a".to_string())
    }
}

enum Event {
    ShiftBegin(String),
    FallAsleep,
    WakeUp,
}