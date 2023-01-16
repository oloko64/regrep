// use rayon::prelude::*;
use std::{
    io::{self, BufRead},
    process::exit, fmt::{Display, Formatter, Error},
    // sync::Mutex,
};

struct Matches {
    line: String,
    line_number: usize,
}

impl Matches {
    fn new(line: &str, line_number: usize, matcher: &str) -> Self {
        Self {
            line: line.replace(matcher, &format!("\x1b[94m{matcher}\x1b[0m")),
            line_number,
        }
    }
}

impl Display for Matches {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}: {}", self.line_number, self.line)
    }
}

fn main() {
    if let Some(matched) = match_in_file(&read_lines_stdin(), &get_match_arg()) {
        for line in matched {
            println!("{line}");
        }
    }
}

fn read_lines_stdin() -> Vec<String> {
    io::stdin()
        .lock()
        .lines()
        .into_iter()
        .map(Result::unwrap)
        .collect()
}

fn match_in_file(lines: &[String], matcher_list: &[String]) -> Option<Vec<Matches>> {
    let mut matches = Vec::new();
    for (index, line) in lines.iter().enumerate() {
        for matcher in matcher_list {
            if line.contains(matcher) {
                matches.push(Matches::new(line, index, matcher));
            }
        }
    }
    if matches.is_empty() {
        None
    } else {
        Some(matches)
    }
}

// Just an example using parallel iterators
// fn match_in_file(lines: &Vec<String>, matcher_list: &Vec<String>) -> Option<Vec<LineMatch>> {
//     let matches = Mutex::new(Vec::new());
//     lines.par_iter().enumerate().for_each(|(index, line)| {
//         matcher_list.par_iter().for_each(|matcher| {
//             if line.contains(matcher) {
//                 let mut guard = matches.lock().unwrap();
//                 guard.push(LineMatch {
//                     line: line
//                         .to_string()
//                         .replace(matcher, &format!("\x1b[94m{}\x1b[0m", matcher)),
//                     line_number: index as i64 + 1,
//                 });
//             }
//         });
//     });
//     if matches.lock().unwrap().is_empty() {
//         None
//     } else {
//         Some(matches.lock().unwrap().to_vec())
//     }
// }

fn get_match_arg() -> Vec<String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("You need to provide at least one argument to match");
        exit(1);
    }
    args[1..].to_vec()
}
