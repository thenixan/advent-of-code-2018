use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Error;
use std::fmt::Formatter;
use std::ops::Range;
use std::str::FromStr;

use chrono::DateTime;
use chrono::offset::TimeZone;
use chrono::Timelike;
use chrono::Utc;

use days::print_header;
use days::read_file_to_vec;

pub fn run_first_task() {
    print_header(4, 1);
    match read_file_to_vec("days/4/input", |s| s.parse::<Record>())
        .map(|records| {
            records
                .into_iter()
                .fold(Vec::new(), |mut v, item| {
                    v.push(item);
                    v.sort_by(|l, r| l.timestamp.cmp(&r.timestamp));
                    v
                })
        })
        .map(|records| {
            let mut i = 0;
            let result = &mut Vec::new();
            while i < records.len() {
                let record = &records[i];
                match &record.event {
                    Event::ShiftBegin(id) => {
                        let current_id = id;
                        let current_timestamp = record.timestamp;
                        let mut periods = Vec::new();
                        i += 1;
                        while i < records.len() {
                            let fall_asleep = &records[i];
                            match fall_asleep.event {
                                Event::ShiftBegin(_) => break,
                                Event::FallAsleep => {
                                    i += 1;
                                    let awake = &records[i];
                                    match awake.event {
                                        Event::WakeUp => {
                                            periods.push(SleepPeriod::new(fall_asleep.timestamp, awake.timestamp));
                                            i += 1;
                                        }
                                        _ => panic!("incorrect sequence of {}", current_id)
                                    }
                                }
                                _ => panic!("incorrect sequence of {}", current_id)
                            }
                        }
                        result.push(Shift::new(current_id.to_string(), current_timestamp, periods.to_vec()));
                    }
                    _ => i += 1,
                }
            }
            result.to_vec()
        })
        .map(|records| {
            records
                .iter()
                .fold(HashMap::new(), |mut s, shift| {
                    {
                        let (sleep_length, guard_shifts) = s
                            .entry(&shift.guard_id)
                            .or_insert_with(|| (0, vec!()));
                        *sleep_length += shift.sleep_length();
                        guard_shifts.push(shift);
                    }
                    s
                })
                .iter()
                .max_by(|(_, (l_sum, _)), (_, (r_sum, _))| {
                    if &l_sum < &r_sum {
                        Ordering::Less
                    } else if &l_sum > &r_sum {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                })
                .map_or(Err("no maximum".to_string()), |(guard_id, (_, shift))| {
                    Ok((guard_id, shift))
                })
                .map(|(guard_id, shifts)| {
                    (guard_id, shifts
                        .iter()
                        .flat_map(|shift| &shift.sleep_periods)
                        .map(|p| into_range(&p))
                        .collect::<Vec<Range<u32>>>())
                })
                .map(|(guard_id, minutes)| {
                    (guard_id, find_most_overlap(&minutes))
                })
                .and_then(|(guard_id, minute)| {
                    if let Ok(n) = guard_id.parse::<u32>() {
                        Ok(n * minute)
                    } else {
                        Err("cannot parse".to_string())
                    }
                })
        })
        .and_then(|r| r)
        {
            Ok(result) => { println!("Result: {}", result) }
            Err(_) => { println!("No maximums") }
        };
}

fn find_most_overlap(v: &Vec<Range<u32>>) -> u32 {
    let mut result = 0;
    let mut value: u32 = 0;
    v.iter().for_each(|r| {
        for i in r.start..r.end {
            let sum = v.iter().fold(0, |s, item| {
                if item.start <= i && i < item.end {
                    s + 1
                } else {
                    s
                }
            });
            if sum > result {
                result = sum;
                value = i;
            }
        }
    });
    value
}

#[derive(Clone)]
struct SleepPeriod {
    started: DateTime<Utc>,
    finished: DateTime<Utc>,
}

impl SleepPeriod {
    fn new(start: DateTime<Utc>, finish: DateTime<Utc>) -> SleepPeriod {
        SleepPeriod { started: start, finished: finish }
    }

    fn duration(self) -> i64 {
        let d = self.finished.signed_duration_since(self.started);
        d.num_hours() * 60 + d.num_minutes()
    }
}

#[derive(Clone)]
struct Shift {
    start_time: DateTime<Utc>,
    guard_id: String,
    sleep_periods: Vec<SleepPeriod>,
}

impl Shift {
    fn new(id: String, start: DateTime<Utc>, periods: Vec<SleepPeriod>) -> Shift {
        Shift {
            start_time: start,
            guard_id: id,
            sleep_periods: periods,
        }
    }

    fn sleep_length(&self) -> i64 {
        self.sleep_periods
            .iter()
            .map(|i| i.to_owned().duration())
            .fold(0, |sum, i| {
                sum + i
            })
    }
}

struct Record {
    timestamp: DateTime<Utc>,
    event: Event,
}

impl Record {
    fn new(timestamp: DateTime<Utc>, event: Event) -> Record {
        Record { timestamp, event }
    }
}

fn into_range(s: &SleepPeriod) -> Range<u32> {
    Range { start: s.started.minute(), end: s.finished.minute() }
}

impl FromStr for Record {
    type Err = String;

    fn from_str(s: &str) -> Result<Record, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<_>>();
        let timestamp = Utc.datetime_from_str(&[parts[0], parts[1]].join(" "), "[%Y-%m-%d %H:%M]");
        let event = parts[2..].join(" ").parse::<Event>();
        match (timestamp, event) {
            (Ok(t), Ok(e)) => Ok(Record::new(t, e)),
            _ => Err("cannot parse record".to_string()),
        }
    }
}

impl fmt::Display for Shift {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "id: {}, ts: {}", self.guard_id, self.start_time)
    }
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let event_name = match &self.event {
            Event::WakeUp => "wake_up".to_string(),
            Event::FallAsleep => "fall_asleep".to_string(),
            Event::ShiftBegin(s) => format!("shift_begin: {}", s.to_string()),
        };
        write!(f, "event: {}, timestamp: {}", event_name, self.timestamp)
    }
}

enum Event {
    ShiftBegin(String),
    FallAsleep,
    WakeUp,
}

impl FromStr for Event {
    type Err = String;

    fn from_str(s: &str) -> Result<Event, Self::Err> {
        if s == "wakes up" {
            Ok(Event::WakeUp)
        } else if s == "falls asleep" {
            Ok(Event::FallAsleep)
        } else {
            let splitted = s.split_whitespace().collect::<Vec<_>>();
            let guard_number = splitted.get(1).unwrap().chars().skip(1).collect::<String>();
            Ok(Event::ShiftBegin(guard_number))
        }
    }
}