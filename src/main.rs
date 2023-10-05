use core::fmt;
use nucleo::{Config, Matcher, Utf32Str};
use std::{
    env,
    io::{self, BufRead},
    process::exit,
    time::Instant,
};

#[derive(Debug)]
enum MatcherType {
    Fuzzy,
    FuzzyIndices,
    FuzzyGreedy,
    FuzzyGreedyIndices,
    Substring,
    SubstringIndices,
    Exact,
    ExactIndices,
    Prefix,
    PrefixIndices,
    Postfix,
    PostfixIndices,
}

impl fmt::Display for MatcherType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[rustfmt::skip]
pub fn main() {
    let args: Vec<String> = env::args().collect();

    // arg parsing (manually)
    let mut arg_iter = args.iter().skip(1);
    let mut patterns = vec![];

    while let Some(arg) = arg_iter.next() {
        patterns.push(arg.to_string());
    }

    if patterns.is_empty() {
        eprintln!("Usage: echo <piped_input> | fz <pattern>");
        eprintln!("    For Nushell try this:");
        eprintln!(r#"    ["foo-bar", "baz-brr"] | to text | fz foo bar"#);
        exit(1);
    }

    let config = Config::DEFAULT;
    let mut matcher = Box::new(nucleo::Matcher::new(config));

    println!(
        "{:22} | {}\t\t| {:3} | {}",
        "method", "match", "score", "elapsed"
    );
    println!("-----------------------|----------------|-------|--------");
    let stdin = io::stdin();
    let start_time = Instant::now();
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            let mut indicies = vec![];
            let utf32str_line = Utf32Str::Ascii(line.as_bytes());
            for pattern in &patterns {
            let utf32str_pattern = Utf32Str::Ascii(&pattern.as_bytes());

            do_matching(&mut matcher, MatcherType::Fuzzy, utf32str_line, utf32str_pattern, &mut indicies);
            do_matching(&mut matcher, MatcherType::FuzzyIndices, utf32str_line, utf32str_pattern, &mut indicies);
            do_matching(&mut matcher, MatcherType::FuzzyGreedy, utf32str_line, utf32str_pattern, &mut indicies);
            do_matching(&mut matcher, MatcherType::FuzzyGreedyIndices, utf32str_line, utf32str_pattern, &mut indicies);
            do_matching(&mut matcher, MatcherType::Substring, utf32str_line, utf32str_pattern, &mut indicies);
            do_matching(&mut matcher, MatcherType::SubstringIndices, utf32str_line, utf32str_pattern, &mut indicies);
            do_matching(&mut matcher, MatcherType::Exact, utf32str_line, utf32str_pattern, &mut indicies);
            do_matching(&mut matcher, MatcherType::ExactIndices, utf32str_line, utf32str_pattern, &mut indicies);
            do_matching(&mut matcher, MatcherType::Prefix, utf32str_line, utf32str_pattern, &mut indicies);
            do_matching(&mut matcher, MatcherType::PrefixIndices, utf32str_line, utf32str_pattern, &mut indicies);
            do_matching(&mut matcher, MatcherType::Postfix, utf32str_line, utf32str_pattern, &mut indicies);
            do_matching(&mut matcher, MatcherType::PostfixIndices, utf32str_line, utf32str_pattern, &mut indicies);
            }
        }
    }
    eprintln!("\nElapsed: {:?}", start_time.elapsed());
}

fn do_matching(
    matcher: &mut Box<Matcher>,
    matcher_type: MatcherType,
    utf32str_line: Utf32Str,
    utf32str_pattern: Utf32Str,
    indicies: &mut Vec<u32>,
) {
    let timing = Instant::now();
    let (match_score, elapsed) = match matcher_type {
        MatcherType::Fuzzy => (
            matcher.fuzzy_match(utf32str_line, utf32str_pattern),
            timing.elapsed(),
        ),
        MatcherType::FuzzyIndices => (
            matcher.fuzzy_indices(utf32str_line, utf32str_pattern, indicies),
            timing.elapsed(),
        ),
        MatcherType::FuzzyGreedy => (
            matcher.fuzzy_match_greedy(utf32str_line, utf32str_pattern),
            timing.elapsed(),
        ),
        MatcherType::FuzzyGreedyIndices => (
            matcher.fuzzy_indices_greedy(utf32str_line, utf32str_pattern, indicies),
            timing.elapsed(),
        ),
        MatcherType::Substring => (
            matcher.substring_match(utf32str_line, utf32str_pattern),
            timing.elapsed(),
        ),
        MatcherType::SubstringIndices => (
            matcher.substring_indices(utf32str_line, utf32str_pattern, indicies),
            timing.elapsed(),
        ),
        MatcherType::Exact => (
            matcher.exact_match(utf32str_line, utf32str_pattern),
            timing.elapsed(),
        ),
        MatcherType::ExactIndices => (
            matcher.exact_indices(utf32str_line, utf32str_pattern, indicies),
            timing.elapsed(),
        ),
        MatcherType::Prefix => (
            matcher.prefix_match(utf32str_line, utf32str_pattern),
            timing.elapsed(),
        ),
        MatcherType::PrefixIndices => (
            matcher.prefix_indices(utf32str_line, utf32str_pattern, indicies),
            timing.elapsed(),
        ),
        MatcherType::Postfix => (
            matcher.postfix_match(utf32str_line, utf32str_pattern),
            timing.elapsed(),
        ),
        MatcherType::PostfixIndices => (
            matcher.postfix_indices(utf32str_line, utf32str_pattern, indicies),
            timing.elapsed(),
        ),
    };

    if let Some(score) = match_score {
        let matches = highlight_matches(&utf32str_line.to_string(), indicies.as_slice());
        println!(
            "{:22} | {matches}\t| {score:5} | {:?}",
            matcher_type.to_string(),
            elapsed
        );
        indicies.clear();
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
