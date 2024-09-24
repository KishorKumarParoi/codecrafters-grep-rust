use std::env;
use std::io;
use std::process;
#[derive(Debug, PartialEq, Clone)]
enum Character {
    Any,
    Literal(char),
    DecimalDigit,
    Word,
    Group(Vec<Character>),
    NegativeGroup(Vec<Character>),
    Optional(Box<Character>),
    RepeatedOptional(Box<Character>),
    Either((Vec<Character>, Vec<Character>)),
    CaptureGroup(Vec<Character>),
}
#[derive(Debug)]
enum Modifier {
    OneOrMore,
    ZeroOrOne,
    ZeroOrMore,
    Reference(usize),
}
fn parse_pattern(input: &str) -> Vec<Character> {
    let mut output: Vec<Character> = vec![];
    let mut remainder = input;
    while !remainder.is_empty() {
        let (rest, character, modifier) = parse_character(remainder);
        match modifier {
            Some(Modifier::OneOrMore) => {
                // Keep the original item
                let prev = output.last().unwrap().clone();
                // And add it as optional as well
                output.push(Character::RepeatedOptional(Box::new(prev)));
            }
            Some(Modifier::ZeroOrOne) => {
                // Remove the last item
                let prev = output.pop().unwrap();
                // And re-add it as an optional
                output.push(Character::Optional(Box::new(prev)));
            }
            Some(Modifier::ZeroOrMore) => {
                // Remove the last item
                let prev = output.pop().unwrap();
                // And re-add it as a repeated optional
                output.push(Character::RepeatedOptional(Box::new(prev)));
            }
            Some(Modifier::Reference(index)) => {
                let group = output
                    .iter()
                    .filter(|c| matches!(c, Character::CaptureGroup(_)))
                    .nth(index - 1)
                    .unwrap();
                if let Character::CaptureGroup(group) = group {
                    output.extend(group.clone());
                }
            }
            None => {
                output.push(character.expect("Should have a character without modifier"));
            }
        }
        remainder = rest;
    }
    output
}
fn parse_character(input: &str) -> (&str, Option<Character>, Option<Modifier>) {
    match input.chars().next() {
        Some('\\') => (
            &input[2..],
            Some(special_character(&input.chars().nth(1).unwrap())),
            None,
        ),
        Some('\\') => {
            let remainder = &input[2..];
            match &input.chars().nth(1).unwrap() {
                index @ '1'..='9' => (
                    remainder,
                    None,
                    Some(Modifier::Reference(index.to_digit(10).unwrap() as usize)),
                ),
                c => (remainder, Some(special_character(c)), None),
            }
        }
        Some('[') => {
            let mut group = vec![];
            let mut remainder = &input[1..];
            let is_negative = &input[1..2] == "^";
            if is_negative {
                remainder = &remainder[1..];
            }
            while !remainder.is_empty() && !remainder.starts_with(']') {
                let (rest, character, _) = parse_character(remainder);
            let pos = remainder.find(']').expect("closing bracket missing");
            let group = parse_pattern(&remainder[..pos]);
            remainder = &remainder[pos..];
                group.push(character.expect("Should have a character without modifier"));
                remainder = rest;
            }
            if is_negative {
                (
                    remainder.strip_prefix(']').unwrap(),
                    Some(Character::NegativeGroup(group)),
                    None,
                )
            let group = if is_negative {
                Character::NegativeGroup(group)
            } else {
                (
                    remainder.strip_prefix(']').unwrap(),
                    Some(Character::Group(group)),
                    None,
                )
            }
                Character::Group(group)
            };
            (&remainder[1..], Some(group), None)
        }
        Some('(') => {
            let mut remainder = &input[1..];
            let mut first = vec![];
            while !remainder.starts_with('|') {
                let (rest, character, _) = parse_character(remainder);
                first.push(character.expect("Should have a character without modifier"));
                remainder = rest;
            }
            // Strip '|' prefix
            remainder = &remainder[1..];
            let mut second = vec![];
            while !remainder.starts_with(')') {
                let (rest, character, _) = parse_character(remainder);
            let left = if let Some(pos) = remainder.find('|') {
                let pattern = &remainder[..pos];
                remainder = &remainder[pos + 1..];
                Some(parse_pattern(pattern))
            } else {
                None
            };
                second.push(character.expect("Should have a character without modifier"));
            let pos = remainder.find(')').expect("closing paren missing");
            let right = parse_pattern(&remainder[..pos]);
                remainder = rest;
            }
            let group = match left {
                Some(left) => vec![Character::Either((left, right))],
                _ => right,
            };
            (
                remainder.strip_prefix(')').unwrap(),
                Some(Character::Either((first, second))),
                &remainder[pos + 1..],
                Some(Character::CaptureGroup(group)),
                None,
            )
        }
        Some('+') => (&input[1..], None, Some(Modifier::OneOrMore)),
        Some('?') => (&input[1..], None, Some(Modifier::ZeroOrOne)),
        Some('*') => (&input[1..], None, Some(Modifier::ZeroOrMore)),
        Some('.') => (&input[1..], Some(Character::Any), None),
        Some(c) => (&input[1..], Some(Character::Literal(c)), None),
        _ => panic!("Unhandled pattern: {}", input),
    }
}
fn special_character(input: &char) -> Character {
    match input {
        'd' => Character::DecimalDigit,
        'w' => Character::Word,
        '\\' => Character::Literal('\\'),
        _ => panic!("Unsupported special char: {}", input),
    }
}
fn to_match_result(input: &str, has_match: bool) -> Result<&str, &str> {
    if has_match {
        Ok(&input[1..])
    } else {
        Err(&input[1..])
    }
}
fn match_character(input: &str, character: Character) -> Result<&str, &str> {
    if input.is_empty() {
        return Ok("");
    }
    let mut input = input;
    let ch = input.chars().next().unwrap();
    match character {
        Character::Any => to_match_result(input, true),
        Character::Literal(c) => to_match_result(input, c == ch),
        Character::DecimalDigit => to_match_result(input, ch.is_ascii_digit()),
        Character::Word => to_match_result(input, ch == '_' || ch.is_ascii_alphanumeric()),
        Character::Group(items) => to_match_result(
            input,
            items
                .iter()
                .any(|i| match_character(input, i.clone()).is_ok()),
        ),
        Character::NegativeGroup(items) => to_match_result(
            input,
            !items
                .iter()
                .any(|i| match_character(input, i.clone()).is_ok()),
        ),
        Character::Optional(c) => {
            if match_character(input, *c.clone()).is_ok() {
                Ok(&input[1..])
            } else {
                Ok(input)
            }
        }
        Character::RepeatedOptional(c) => {
            loop {
                if match_character(input, *c.clone()).is_ok() {
                    if input.is_empty() {
                        break;
                    }
                    input = &input[1..];
                } else {
                    break;
                }
            }
            Ok(input)
        }
        Character::Either((left, right)) => {
            if let Ok(res) = check_branch(input, left) {
                Ok(res)
            } else if let Ok(res) = check_branch(input, right) {
                Ok(res)
            } else {
                Err(input)
            }
        }
        Character::CaptureGroup(group) => {
            if let Ok(res) = check_branch(input, group) {
                Ok(res)
            } else {
                Err(&input[1..])
            }
        }
    }
}
fn check_branch(input: &str, chars: Vec<Character>) -> Result<&str, &str> {
    let mut input_mut = input;
    for ch in chars {
        match match_character(input_mut, ch) {
            Ok(res) => {
                input_mut = res;
            }
            Err(_) => {
                return Err(input);
            }
        }
    }
    Ok(input_mut)
}
fn match_pattern(input_line: &str, pattern: &str) -> bool {
    let mut start_anchor = false;
    let mut end_anchor = false;
    let mut pattern = pattern;
    if pattern.starts_with('^') {
        start_anchor = true;
        pattern = &pattern[1..];
    }
    if pattern.ends_with('$') {
        end_anchor = true;
        pattern = &pattern[..pattern.len() - 1];
    }
    let pattern = parse_pattern(pattern);
    let mut input = input_line;
    loop {
        'inner: loop {
            for idx in 0..pattern.len() {
                let ch = pattern.get(idx).unwrap();
                match match_character(input, ch.clone()) {
                    Ok(res) => {
                        if res.is_empty() {
                            // End of the pattern, match is succesful
                            return idx == pattern.len() - 1;
                        }
                        input = res;
                    }
                    Err(res) => {
                        if start_anchor {
                            // Needed to match from the start
                            return false;
                        }
                        if res.is_empty() {
                            // End of the input, but didn't get the match
                            return false;
                        }
                        input = res;
                        // Reset the pattern
                        break 'inner;
                    }
                }
            }
            // Whole pattern was matched and there's still more input left
            // Match will fail if end anchor was set
            return !end_anchor;
        }
    }
}
// Usage: echo <input_text> | your_grep.sh -E <pattern>
fn main() {
    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }
    let pattern = env::args().nth(2).unwrap();
    let pattern = dbg!(env::args().nth(2).unwrap());
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    if match_pattern(input_line.trim_end(), &pattern) {
    if match_pattern(dbg!(input_line.trim_end()), &pattern) {
        eprintln!("Success");
        process::exit(0)
    } else {
        eprintln!("Failure");
        process::exit(1)
    }
}