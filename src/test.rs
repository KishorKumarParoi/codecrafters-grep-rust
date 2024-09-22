use std::env;
use std::io;
use std::process;
use std::str::Chars;
#[derive(Debug)]
enum Pattern {
    Literal(char),
    Digit,
    Alphanumeric,
    Group(bool, String),
}
fn match_literal(chars: &mut Chars, literal: char) -> bool {
    let c = chars.next();
    c.is_some_and(|c| c == literal)
}
fn match_digit(chars: &mut Chars) -> bool {
    let c = chars.next();
    if c.is_none() {
        return false;
    }
    c.unwrap().is_digit(10)
}
fn match_alphanumeric(chars: &mut Chars) -> bool {
    let c = chars.next();
    c.is_some_and(|c| c.is_alphanumeric())
}
fn match_group(chars: &mut Chars, group: &str) -> bool {
    let c = chars.next();
    c.is_some_and(|c| group.contains(c))
}
fn match_pattern(input_line: &str, pattern: &str) -> bool {
    let patterns = build_patterns(pattern);
    let input_line = input_line.trim_matches('\n');
    'input_iter: for i in 0..input_line.len() {
        let input = &input_line[i..];
        let mut iter = input.chars();
        for pattern in patterns.iter() {
            match pattern {
                Pattern::Literal(l) => {
                    if !match_literal(&mut iter, *l) {
                        continue 'input_iter;
                    }
                }
                Pattern::Digit => {
                    if !match_digit(&mut iter) {
                        continue 'input_iter;
                    }
                }
                Pattern::Alphanumeric => {
                    if !match_alphanumeric(&mut iter) {
                        continue 'input_iter;
                    }
                }
                Pattern::Group(positive, group) => {
                    if match_group(&mut iter, group) != *positive {
                        continue 'input_iter;
                    }
                }
            }
        }
        return true;
    }
    return false;
}
fn build_group_pattern(iter: &mut Chars) -> (bool, String) {
    let mut group = String::new();
    let mut positive = true;
    if iter.clone().next().is_some_and(|c| c == '^') {
        positive = false;
        iter.next();
    }
    loop {
        let member = iter.next();
        if member.is_none() {
            panic!("Incomplete character group");
        }
        let member = member.unwrap();
        if member != ']' {
            group.push(member);
            continue;
        }
        break;
    }
    (positive, group)
}
fn build_patterns(pattern: &str) -> Vec<Pattern> {
    let mut iter = pattern.chars();
    let mut patterns = Vec::new();
    loop {
        let current = iter.next();
        if current.is_none() {
            break;
        }
        patterns.push(match current.unwrap() {
            '\\' => {
                let special = iter.next();
                if special.is_none() {
                    panic!("Incomplete special character")
                }
                match special.unwrap() {
                    'd' => Pattern::Digit,
                    'w' => Pattern::Alphanumeric,
                    '\\' => Pattern::Literal('\\'),
                    _ => panic!("Invalid special character"),
                }
            }
            '[' => {
                let (positive, group) = build_group_pattern(&mut iter);
                Pattern::Group(positive, group)
            }
            l => Pattern::Literal(l),
        })
    }
    patterns
}
// Usage: echo <input_text> | your_grep.sh -E <pattern>
fn main() {
    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }
    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    if match_pattern(&input_line, &pattern) {
        process::exit(0)
    } else {
        process::exit(1)
    }
}
src/main.rs
View on GitHub
use std::env;
use std::io;
use std::process;
use std::str::Chars;
#[derive(Debug)]
enum Pattern {
    Literal(char),
    Digit,
    Alphanumeric,
    Group(bool, String),
}
fn match_literal(chars: &mut Chars, literal: char) -> bool {
    let c = chars.next();
    c.is_some_and(|c| c == literal)
}
fn match_digit(chars: &mut Chars) -> bool {
    let c = chars.next();
    if c.is_none() {
        return false;
    }
    c.unwrap().is_digit(10)
}
fn match_alphanumeric(chars: &mut Chars) -> bool {
    let c = chars.next();
    c.is_some_and(|c| c.is_alphanumeric())
}
fn match_group(chars: &mut Chars, group: &str) -> bool {
    let c = chars.next();
    c.is_some_and(|c| group.contains(c))
}
fn match_pattern(input_line: &str, pattern: &str) -> bool {
    if pattern.chars().count() == 1 {
        return input_line.contains(pattern);
    } else if pattern == "\\d" {
        return input_line.chars().any(|c| c.is_digit(10));
    } else if pattern == "\\w" {
        return input_line.chars().any(|c| c.is_alphanumeric());
    } else if pattern.starts_with('[') && pattern.ends_with(']') {
        if pattern.len() >= 3 && pattern.chars().nth(1).unwrap() == '^' {
            let chars = pattern[2..pattern.len() - 1].chars().collect::<Vec<char>>();
            return chars.iter().all(|c| !input_line.contains(*c));
        } else {
            let chars = pattern[1..pattern.len() - 1].chars().collect::<Vec<char>>();
            return chars.iter().any(|c| input_line.contains(*c));
    let patterns = build_patterns(pattern);
    let input_line = input_line.trim_matches('\n');
    'input_iter: for i in 0..input_line.len() {
        let input = &input_line[i..];
        let mut iter = input.chars();
        for pattern in patterns.iter() {
            match pattern {
                Pattern::Literal(l) => {
                    if !match_literal(&mut iter, *l) {
                        continue 'input_iter;
                    }
                }
                Pattern::Digit => {
                    if !match_digit(&mut iter) {
                        continue 'input_iter;
                    }
                }
                Pattern::Alphanumeric => {
                    if !match_alphanumeric(&mut iter) {
                        continue 'input_iter;
                    }
                }
                Pattern::Group(positive, group) => {
                    if match_group(&mut iter, group) != *positive {
                        continue 'input_iter;
                    }
                }
            }
        }
    } else {
        panic!("Unhandled pattern: {}", pattern)
        return true;
    }
    return false;
}
// fn find_digits(input_line: &str) -> Vec<char> {
//     input_line.chars().filter(|c| c.is_digit(10)).collect()
// }
// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {
    // You can use print statements as follows for debugging,
    // they'll be visible when running tests.
    println!("Logs from your program will appear here!");
    // Print all command-line arguments
    for (i, arg) in env::args().enumerate() {
        println!("Argument {}: {}", i, arg);
fn build_group_pattern(iter: &mut Chars) -> (bool, String) {
    let mut group = String::new();
    let mut positive = true;
    if iter.clone().next().is_some_and(|c| c == '^') {
        positive = false;
        iter.next();
    }
    loop {
        let member = iter.next();
        if member.is_none() {
            panic!("Incomplete character group");
        }
        let member = member.unwrap();
        if member != ']' {
            group.push(member);
            continue;
        }
        break;
    }
    (positive, group)
}
fn build_patterns(pattern: &str) -> Vec<Pattern> {
    let mut iter = pattern.chars();
    let mut patterns = Vec::new();
    loop {
        let current = iter.next();
        if current.is_none() {
            break;
        }
        patterns.push(match current.unwrap() {
            '\\' => {
                let special = iter.next();
                if special.is_none() {
                    panic!("Incomplete special character")
                }
                match special.unwrap() {
                    'd' => Pattern::Digit,
                    'w' => Pattern::Alphanumeric,
                    '\\' => Pattern::Literal('\\'),
                    _ => panic!("Invalid special character"),
                }
            }
            '[' => {
                let (positive, group) = build_group_pattern(&mut iter);
                Pattern::Group(positive, group)
            }
            l => Pattern::Literal(l),
        })
    }
    patterns
}
// Usage: echo <input_text> | your_grep.sh -E <pattern>
fn main() {
    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }
    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    //  Uncomment this block to pass the first stage
    if match_pattern(&input_line, &pattern) {
        println! {"{}, {}", &input_line, &pattern};
        process::exit(0)
    } else {
        process::exit(1)
    }
    // if (pattern.to_string() == "\\d") {
    //     let digits = find_digits(&input_line);
    //     println!("{:?}", digits);
    // } else {
    //     process::exit(1)
    // }
}

Collapse example
avatar
andy31415
Concise

1 comments
8 months ago

Collapse example
src/lib.rs
use std::{collections::HashSet, str::FromStr};
use map_macro::hash_set;
use tracing::{instrument, trace};
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Pattern {
    ExactChar(char),
    AnyChar,
    Digit,        // 0-9
    AlphaNumeric, // a-zA-Z0-9_
    Sequence(Vec<Pattern>),
    Repeated {
        min: usize,
        max: Option<usize>,
        pattern: Box<Pattern>,
    },
    OneOf(Vec<Pattern>),
    CharacterSet {
        chars: String,
        negated: bool,
    },
}
trait CharOperations {
    fn first_char(&self) -> Option<char>;
    fn first_char_in(&self, options: &str) -> bool;
    fn skip_first_char(&self) -> Self;
}
impl CharOperations for &str {
    fn first_char(&self) -> Option<char> {
        return self.chars().next();
    }
    fn first_char_in(&self, options: &str) -> bool {
        match self.first_char() {
            Some(c) => options.contains(c),
            None => false,
        }
    }
    fn skip_first_char(&self) -> Self {
        &self[1..]
    }
}
impl FromStr for Pattern {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut char_iterator = s.chars();
        let mut items = Vec::new();
        while let Some(c) = char_iterator.next() {
            let el = match c {
                '\\' => match char_iterator.next() {
                    Some('w') => Pattern::AlphaNumeric,
                    Some('d') => Pattern::Digit,
                    Some(c) => Pattern::ExactChar(c), // assume an escape
                    None => return Err(format!("Unterminated escape in {:?}", s)),
                },
                '.' => Pattern::AnyChar,
                '*' => {
                    // need to grab last item and repeat
                    match items.pop() {
                        Some(p) => Pattern::Repeated {
                            min: 0,
                            max: None,
                            pattern: Box::new(p),
                        },
                        None => return Err("Invalid repeat".into()),
                    }
                }
                '[' => {
                    let mut chars = String::new();
                    let mut found_end = false;
                    let mut negated = false;
                    for c2 in char_iterator.by_ref() {
                        match c2 {
                            '^' if chars.is_empty() => negated = true,
                            // TODO: should we handle escapes here?
                            ']' => {
                                found_end = true;
                                break;
                            }
                            other => chars.push(other),
                        }
                    }
                    if !found_end {
                        return Err("Unterminated '[' pattern".into());
                    }
                    Pattern::CharacterSet { chars, negated }
                }
                e => Pattern::ExactChar(e),
            };
            items.push(el);
        }
        if items.len() == 1 {
            return Ok(items.pop().expect("has an element"));
        }
        Ok(Pattern::Sequence(items))
    }
}
impl Pattern {
    #[instrument]
    pub fn match_str<'a>(&'_ self, data: &'a str) -> HashSet<&'a str> {
        trace!("Matching starts");
        match self {
            Pattern::AnyChar if data.first_char().is_some() => hash_set! {data.skip_first_char()},
            Pattern::ExactChar(c) if data.first_char() == Some(*c) => {
                hash_set! {data.skip_first_char()}
            }
            Pattern::Digit if data.first_char_in("0123456789") => {
                hash_set! {data.skip_first_char()}
            }
            Pattern::AlphaNumeric
                if data.first_char_in(
                    "_0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ",
                ) =>
            {
                hash_set! {data.skip_first_char()}
            }
            Pattern::Sequence(sub_patterns) => {
                let mut remaining = hash_set! {data};
                for sub_pattern in sub_patterns {
                    let mut next_remaining = HashSet::new();
                    for r in remaining.iter() {
                        next_remaining.extend(sub_pattern.match_str(r))
                    }
                    remaining = next_remaining
                }
                remaining
            }
            Pattern::CharacterSet { chars, negated } => {
                trace!(
                    "TEST: {} and {} (for {})",
                    data.first_char_in(chars),
                    negated,
                    chars
                );
                if !data.is_empty() && data.first_char_in(chars) != *negated {
                    hash_set! {data.skip_first_char()}
                } else {
                    HashSet::new()
                }
            }
            Pattern::OneOf(sub_patterns) => {
                let mut result = HashSet::new();
                for sub_pattern in sub_patterns {
                    result.extend(sub_pattern.match_str(data))
                }
                result
            }
            Pattern::Repeated { min, max, pattern } => {
                let mut results: HashSet<&str> = HashSet::new();
                let mut remaining = vec![data];
                let mut count = 0;
                while !remaining.is_empty() {
                    if count >= *min {
                        // all matches appended
                        results.extend(remaining.iter());
                    }
                    count += 1;
                    // did we reach max count
                    if max.map(|m| m < count).unwrap_or(false) {
                        break;
                    }
                    // try matching for the pattern and append
                    let mut new_ends = Vec::new();
                    for r in remaining {
                        for x in pattern.match_str(r) {
                            if results.contains(x) {
                                continue; // already considered
                            }
                            new_ends.push(x);
                        }
                    }
                    remaining = new_ends;
                }
                results
            }
            _ => HashSet::new(),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_match_str_exact_char() {
        assert_eq!(Pattern::ExactChar('A').match_str("ABC"), hash_set! {"BC"});
        assert!(Pattern::ExactChar('X').match_str("ABC").is_empty());
        assert_eq!(Pattern::ExactChar('C').match_str("C"), hash_set![""]);
    }
    #[test]
    fn test_match_str_digit() {
        assert_eq!(Pattern::Digit.match_str("123"), hash_set!["23"]);
        assert!(Pattern::Digit.match_str("ABC").is_empty());
        assert_eq!(Pattern::Digit.match_str("9"), hash_set![""]);
    }
    #[test]
    fn test_match_repeated() {
        assert_eq!(
            Pattern::Repeated {
                min: 0,
                max: Some(2),
                pattern: Box::new(Pattern::Digit)
            }
            .match_str("123"),
            hash_set!["123", "23", "3"],
        );
        assert_eq!(
            Pattern::Repeated {
                min: 2,
                max: Some(3),
                pattern: Box::new(Pattern::Digit)
            }
            .match_str("12345"),
            hash_set!["345", "45"]
        );
        assert_eq!(
            Pattern::Repeated {
                min: 2,
                max: None,
                pattern: Box::new(Pattern::Digit)
            }
            .match_str("12345"),
            hash_set!["345", "45", "5", ""]
        );
        assert_eq!(
            Pattern::Repeated {
                min: 2,
                max: None,
                pattern: Box::new(Pattern::Digit)
            }
            .match_str("123ABC"),
            hash_set!["3ABC", "ABC"]
        );
    }
    #[test]
    fn test_match_str_sequence() {
        assert_eq!(
            Pattern::Sequence(vec![
                Pattern::Digit,
                Pattern::ExactChar('Z'),
                Pattern::Digit,
            ])
            .match_str("1Z2XY"),
            hash_set!["XY"]
        );
    }
    #[test_log::test]
    fn test_matches() {
        assert_eq!(
            Pattern::from_str("AB\\d\\dZZ")
                .expect("valid")
                .match_str("AB12ZZCD"),
            hash_set!["CD"]
        );
        assert_eq!(
            Pattern::from_str("..\\dA")
                .expect("valid")
                .match_str("A12A"),
            hash_set![""]
        );
        assert_eq!(
            Pattern::from_str(".*foo")
                .expect("valid")
                .match_str("foobar"),
            hash_set!["bar"]
        );
        assert_eq!(
            Pattern::from_str(".*foo")
                .expect("valid")
                .match_str("somefoobar"),
            hash_set!["bar"]
        );
        assert_eq!(
            Pattern::from_str(".*ZZ.*X")
                .expect("valid")
                .match_str("ABCZZZ12XX"),
            hash_set!["X", ""]
        );
        assert_eq!(
            Pattern::from_str("[abc]*test")
                .expect("valid")
                .match_str("aabbcatest12"),
            hash_set!["12"]
        );
        assert_eq!(
            Pattern::from_str("[^xyz]*xtest")
                .expect("valid")
                .match_str("aabbcaxtest12"),
            hash_set!["12"]
        );
        assert_eq!(
            Pattern::from_str("[^xyz]*test")
                .expect("valid")
                .match_str("aabbcatest12"),
            hash_set!["12"]
        );
        assert_eq!(
            Pattern::from_str("\\d apple")
                .expect("valid")
                .match_str("1 apple"),
            hash_set![""]
        );
    }
}
