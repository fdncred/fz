use nucleo::Utf32Str;
use std::env;
use std::io::{self, BufRead};
use std::process::exit;

pub fn main() {
    let args: Vec<String> = env::args().collect();

    // arg parsing (manually)
    let mut arg_iter = args.iter().skip(1);
    let mut pattern = "".to_string();

    while let Some(arg) = arg_iter.next() {
        pattern = arg.to_string();
    }

    if &pattern == "" {
        eprintln!("Usage: echo <piped_input> | fz <pattern>");
        exit(1);
    }

    let mut matcher = Box::new(nucleo::Matcher::default());

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            let mut indicies = vec![];
            let utf32str_line = Utf32Str::Ascii(line.as_bytes());
            let utf32str_pattern = Utf32Str::Ascii(&pattern.as_bytes());
            if let Some(_score) =
                matcher.fuzzy_indices(utf32str_line, utf32str_pattern, &mut indicies)
            {
                // eprintln!(
                //     "score: {:?}\nline: {:?}\npattern: {:?}\nindicies: {:?}\n",
                //     score, line, pattern, indicies
                // );
                println!("{:8}:", highlight_matches(&line, indicies.as_slice()));
            }
        }
    }
}

fn highlight_matches(line: &str, indices: &[u32]) -> String {
    let mut ret = String::new();
    let mut peekable = indices.iter().peekable();
    for (idx, ch) in line.chars().enumerate() {
        let next_id = **peekable.peek().unwrap_or(&&(line.len() as u32));
        if next_id == (idx as u32) {
            ret.push_str(format!("\u{1b}[31m{}\u{1b}[0m", ch).as_str());
            peekable.next();
        } else {
            ret.push(ch);
        }
    }

    ret
}
