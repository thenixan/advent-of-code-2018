use std::io::Read;

use unic_char_range::CharRange;

use days::print_header;
use days::read_file;

pub fn run_first_task() {
    print_header(5, 1);
    match read_file("days/5/input")
        .map(|mut reader| {
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
        .flat_map(|c| {
            match read_file("days/5/input") {
                Ok(x) => Ok((c, x)),
                Err(e) => Err(e.to_string()),
            }
        })
        .flat_map(|(c, mut reader)| {
            let mut result = Vec::<u8>::new();
            match reader.read_to_end(&mut result) {
                Ok(_) => Ok((c, result)),
                Err(e) => Err(e.to_string()),
            }
        })
        .map(|(c, v)| {
            v.into_iter()
                .map(|p| p as char)
                .filter(|p| p.to_ascii_lowercase() != c)
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
    c1.to_ascii_lowercase() == c2.to_ascii_lowercase() && *c1 != *c2
}


#[cfg(test)]
mod tests {
    use test::Bencher;

    use days::fifth::collapse;

    #[test]
    fn test_collapse() {
        assert_eq!("dabCBAcaDA".chars().collect::<Vec<_>>(), collapse("dabAcCaCBAcCcaDA".chars().collect::<Vec<_>>()));
    }

    #[bench]
    fn bench_collapse(b: &mut Bencher) {
        b.iter(|| collapse("dabAcCaCBAcCcaDA".chars().collect::<Vec<_>>()));
    }
}