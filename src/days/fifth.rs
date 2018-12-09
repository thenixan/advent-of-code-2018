use std::io::Read;

use unic_char_range::CharRange;

use days::print_header;
use days::read_file_to_vec_by_char;

pub fn run_first_task() {
    print_header(5, 1);
    match read_file_to_vec_by_char("days/5/input", |mut reader| {
        let mut result = Vec::<u8>::new();
        reader.read_to_end(&mut result).unwrap();
        result
    })
        .map(|vec| {
            vec.iter().map(|u| *u as char).collect::<Vec<_>>()
        })
        .map(|vec| collapse(vec))
        .map(|v| v.len()) {
        Ok(x) => println!("Result: {}", x),
        Err(e) => println!("{}", e),
    };
}

pub fn run_second_task() {
    print_header(5, 2);
    match CharRange::closed('a', 'z')
        .iter()
        .map(|c| {
            let data = read_file_to_vec_by_char("days/5/input", |mut reader| {
                let mut result = Vec::<u8>::new();
                reader.read_to_end(&mut result).unwrap();
                result
            });
            (c, data)
        })
        .map(|(c, v)| (c, v.unwrap()))
        .map(|(c, v)| {
            v.into_iter()
                .map(|p| p as char)
                .filter(|p| p.to_lowercase().to_string() != c.to_string())
                .collect::<Vec<_>>()
        })
        .map(|v| {
            collapse(v)
        })
        .map(|v| {
            v.len()
        })
        .min() {
        Some(x) => println!("Result: {}", x),
        None => println!("No minimum found"),
    };
}

fn collapse(mut vec: Vec<char>) -> Vec<char> {
    let mut i = 0;
    while i < vec.len() - 1 {
        let this = vec[i];
        let next = vec[i + 1];
        if compare_chars_ignore_case(&this, &next) {
            vec.remove(i + 1);
            vec.remove(i);
            if i > 0 { i -= 1 }
        } else {
            i += 1;
        }
    }
    vec
}


fn compare_chars_ignore_case(c1: &char, c2: &char) -> bool {
    c1.to_lowercase().to_string() == c2.to_lowercase().to_string() && *c1 != *c2
}
