use std::{
    io::{self, BufRead},
    process::exit,
};

fn main() {
    match match_in_file(read_lines_stdin(), get_match_arg()) {
        Some(matched) => println!("{}", matched.join("\n")),
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

fn match_in_file(lines: Vec<String>, matcher_list: Vec<String>) -> Option<Vec<String>> {
    let mut matches = Vec::new();
    for line in lines {
        matcher_list.iter().for_each(|matcher| {
            if line.contains(matcher) {
                matches.push(line.to_string());
            }
        });
    }
    if matches.len() > 0 {
         Some(matches)
    } else {
         None
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
