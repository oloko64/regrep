// use rayon::prelude::*;
use std::io::{self, BufRead};

mod arg_parser;

fn main() {
    let args = arg_parser::parse_args().expect("Failed to parse arguments");
    let mut lines = read_lines_stdin();
    let mut matcher = args.match_string;

    if args.insensitive {
        lines
            .iter_mut()
            .for_each(|line| *line = line.to_lowercase());
        matcher
            .iter_mut()
            .for_each(|matcher| *matcher = matcher.to_lowercase());
    }

    match_in_file(&lines, &matcher);
}

fn read_lines_stdin() -> Vec<String> {
    io::stdin()
        .lock()
        .lines()
        .into_iter()
        .map(Result::unwrap)
        .collect()
}

fn print_match(line: &str, line_number: usize, matcher: &str) {
    let line = line.replace(matcher, &format!("\x1b[94m{matcher}\x1b[0m"));
    println!("{line_number}: {line}");
}

fn match_in_file(lines: &[String], matcher_list: &[String]) {
    for (index, line) in lines.iter().enumerate() {
        for matcher in matcher_list {
            if line.contains(matcher) {
                print_match(line, index, matcher);
            }
        }
    }
}
