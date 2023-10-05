use core::fmt;
use nucleo::{pattern::CaseMatching, pattern::Pattern, Config, Matcher, Utf32Str};
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

    let stdin = io::stdin();
    let mut input = vec![];
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            input.push(line);
        }
    }

    let start_time = Instant::now();
    // This matching function uses each type of matching technique
    type_1_matching(&patterns, &input);
    eprintln!("\nType 1 Elapsed: {:?}", start_time.elapsed());

    // This matching uses a nucleo Pattern for matching.
    type_2_matching(&patterns, &input);
    eprintln!("Type 2 Elapsed: {:?}", start_time.elapsed());
}

fn type_2_matching(patterns: &Vec<String>, input: &Vec<String>) {
    let patterns_as_str = patterns.join(" ");
    let pat = Pattern::parse(&patterns_as_str, CaseMatching::Smart);
    let config = Config::DEFAULT;
    let mut matcher = nucleo::Matcher::new(config);

    for line in input {
        println!("line: {line}");
        let mut vec_of_indices = vec![];
        let matches = pat.match_list(vec![line.clone()], &mut matcher);
        if matches.is_empty() {
            println!("no matches");
        }
        for (hit, score) in matches {
            println!("{hit}: {score}");
        }

        let utf32str_line = Utf32Str::Ascii(line.as_bytes());
        // let score = pat.score(utf32str_line, &mut matcher);
        // println!("score: {:?}", score);

        let ind_score = pat.indices(utf32str_line, &mut matcher, &mut vec_of_indices);
        let matches = highlight_matches(&utf32str_line.to_string(), vec_of_indices.as_slice());
        println!(
            "score: {:?}, vec_of_indices: {:?}, matches: {}",
            ind_score, vec_of_indices, matches
        );
        println!("");
    }
}

#[rustfmt::skip]
fn type_1_matching(patterns: &Vec<String>, input: &Vec<String>) {
    let config = Config::DEFAULT;
    let mut matcher = Box::new(nucleo::Matcher::new(config));

    println!(
        "{:22} | {}\t\t| {:3} | {}",
        "method", "match", "score", "elapsed"
    );
    println!("-----------------------|----------------|-------|--------");
    for line in input {
        let mut indicies = vec![];
        let utf32str_line = Utf32Str::Ascii(line.as_bytes());
        for pattern in patterns {
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
