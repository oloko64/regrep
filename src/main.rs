use rayon::prelude::*;
use std::{
    io::{self, BufRead},
    process::exit,
    sync::Mutex,
};

#[derive(Clone)]
struct LineMatch {
    line: String,
    line_number: i64,
}

fn main() {
    match match_in_file(&read_lines_stdin(), &get_match_arg()) {
        Some(matched) => {
            for line in matched {
                println!("{}: {}", line.line_number, line.line);
            }
        }
        None => (),
    }
}

fn read_lines_stdin() -> Vec<String> {
    let stdin = io::stdin();
    stdin
        .lock()
        .lines()
        .into_iter()
        .map(|line| line.unwrap())
        .collect()
}

fn match_in_file(lines: &Vec<String>, matcher_list: &Vec<String>) -> Option<Vec<LineMatch>> {
    let matches = Mutex::new(Vec::new());
    lines.par_iter().enumerate().for_each(|(index, line)| {
        matcher_list.par_iter().for_each(|matcher| {
            if line.contains(matcher) {
                let mut guard = matches.lock().unwrap();
                guard.push(LineMatch {
                    line: line
                        .to_string()
                        .replace(matcher, &format!("\x1b[94m{}\x1b[0m", matcher)),
                    line_number: index as i64 + 1,
                });
            }
        });
    });
    if matches.lock().unwrap().is_empty() {
        None
    } else {
        Some(matches.lock().unwrap().to_vec())
    }
}

fn get_match_arg() -> Vec<String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("You need to provide at least one argument to match");
        exit(1);
    }
    args[1..].to_vec()
}
