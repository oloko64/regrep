pub struct Args {
    pub match_string: Vec<String>,
    pub insensitive: bool,
}

pub fn parse_args() -> Result<Args, lexopt::Error> {
    use lexopt::prelude::*;

    let mut match_string = Vec::new();
    let mut insensitive = false;
    let mut parser = lexopt::Parser::from_env();
    while let Some(arg) = parser.next()? {
        match arg {
            Short('i') | Long("insensitive") => {
                insensitive = true;
            }
            Value(val) => {
                match_string.push(val);
                // https://github.com/blyxxyz/lexopt/blob/master/examples/posixly_correct.rs
                match_string.extend(parser.raw_args()?);
            }
            Short('h') | Long("help") => {
                println!("Usage: cat some_file.txt | regrep [-i|--insensitive] MATCH_STRING");
                std::process::exit(0);
            }
            _ => return Err(arg.unexpected()),
        }
    }

    Ok(Args {
        match_string: match_string
            .into_iter()
            .filter_map(|s| s.into_string().ok())
            .collect(),
        insensitive,
    })
}
