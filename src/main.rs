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

    // exact = substring_match_1_ascii, substring_match_ascii_with_prefilter, substring_match_ascii, substring_match_1_non_ascii, substring_match_non_ascii
    // fuzzy_greedy = fuzzy_match_greedy_
    // fuzzy_optimal = fuzzy_match_optimal

    if &pattern == "" {
        eprintln!("Usage: echo <piped_input> | fz <pattern>");
        exit(1);
    }

    let mut matcher = Box::new(nucleo::Matcher::default());

    println!("{:22} | {}\t\t| {:3}", "method", "match", "score");
    println!("-----------------------|----------------|------");
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            let mut indicies = vec![];
            let utf32str_line = Utf32Str::Ascii(line.as_bytes());
            let utf32str_pattern = Utf32Str::Ascii(&pattern.as_bytes());
            // fuzzy_match
            if let Some(score) = matcher.fuzzy_match(utf32str_line, utf32str_pattern) {
                let matches = highlight_matches(&line, &[score.into()]);
                println!("{:22} | {matches}\t| {score:3}", "fuzzy_match");
            }
            // fuzzy_indicies
            if let Some(score) =
                matcher.fuzzy_indices(utf32str_line, utf32str_pattern, &mut indicies)
            {
                // eprintln!(
                //     "score: {:?}\nline: {:?}\npattern: {:?}\nindicies: {:?}\n",
                //     score, line, pattern, indicies
                // );
                let matches = highlight_matches(&line, indicies.as_slice());
                println!("{:22} | {matches}\t| {score:3}", "fuzzy_indices",);
                indicies.clear();
            }
            // fuzzy_match_greedy
            if let Some(score) = matcher.fuzzy_match_greedy(utf32str_line, utf32str_pattern) {
                let matches = highlight_matches(&line, &[score.into()]);
                println!("{:22} | {matches}\t| {score:3}", "fuzzy_match_greedy",);
            }
            // fuzzy_indicies_greedy
            if let Some(score) =
                matcher.fuzzy_indices_greedy(utf32str_line, utf32str_pattern, &mut indicies)
            {
                let matches = highlight_matches(&line, indicies.as_slice());
                println!("{:22} | {matches}\t| {score:3}", "fuzzy_indicies_greedy",);
                indicies.clear();
            }
            // substring_match
            if let Some(score) = matcher.substring_match(utf32str_line, utf32str_pattern) {
                let matches = highlight_matches(&line, indicies.as_slice());
                println!("{:22} | {matches}\t| {score:3}", "substring_match",);
            }
            // substring_indicies
            if let Some(score) =
                matcher.substring_indices(utf32str_line, utf32str_pattern, &mut indicies)
            {
                let matches = highlight_matches(&line, indicies.as_slice());
                println!("{:22} | {matches}\t| {score:3}", "substring_indicies",);
                indicies.clear();
            }
            // exact_match
            if let Some(score) = matcher.exact_match(utf32str_line, utf32str_pattern) {
                let matches = highlight_matches(&line, indicies.as_slice());
                println!("{:22} | {matches}\t| {score:3}", "exact_match",);
            }
            // exact_indicies
            if let Some(score) =
                matcher.exact_indices(utf32str_line, utf32str_pattern, &mut indicies)
            {
                let matches = highlight_matches(&line, indicies.as_slice());
                println!("{:22} | {matches}\t| {score:3}", "exact_indicies",);
                indicies.clear();
            }
            // prefix_match
            if let Some(score) = matcher.prefix_match(utf32str_line, utf32str_pattern) {
                let matches = highlight_matches(&line, indicies.as_slice());
                println!("{:22} | {matches}\t| {score:3}", "prefix_match",);
            }
            // prefix_indicies
            if let Some(score) =
                matcher.prefix_indices(utf32str_line, utf32str_pattern, &mut indicies)
            {
                let matches = highlight_matches(&line, indicies.as_slice());
                println!("{:22} | {matches}\t| {score:3}", "prefix_indicies",);
                indicies.clear();
            }
            // postfix_match
            if let Some(score) = matcher.postfix_match(utf32str_line, utf32str_pattern) {
                let matches = highlight_matches(&line, indicies.as_slice());
                println!("{:22} | {matches}\t| {score:3}", "postfix_match",);
            }
            // postfix_indicies
            if let Some(score) =
                matcher.postfix_indices(utf32str_line, utf32str_pattern, &mut indicies)
            {
                let matches = highlight_matches(&line, indicies.as_slice());
                println!("{:22} | {matches}\t| {score:3}", "postfix_indicies",);
                indicies.clear();
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
