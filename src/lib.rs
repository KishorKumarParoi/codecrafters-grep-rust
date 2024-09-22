use std::str::Chars;

#[derive(Debug, PartialEq)] // Add PartialEq here
pub enum Pattern {
    Literal(char),
    Digit,
    Alphanumeric,
    Group(bool, String),
    Start,
    End,
}

pub fn match_literal(chars: &mut Chars, literal: char) -> bool {
    let c = chars.next();
    c.is_some_and(|c| c == literal)
}

pub fn match_digit(chars: &mut Chars) -> bool {
    let c = chars.next();
    if c.is_none() {
        return false;
    }
    c.unwrap().is_digit(10)
}

pub fn match_alphanumeric(chars: &mut Chars) -> bool {
    let c = chars.next();
    c.is_some_and(|c| c.is_alphanumeric())
}

pub fn match_group(chars: &mut Chars, group: &str) -> bool {
    let c = chars.next();
    c.is_some_and(|c| group.contains(c))
}

pub fn match_pattern(input_line: &str, pattern: &str) -> bool {
    let mut patterns = build_patterns(pattern);
    let mut input_line = input_line.trim_matches('\n').to_string(); // Make input_line mutable

    if let Some(Pattern::End) = patterns.last() {
        input_line = input_line.chars().rev().collect(); // Reverse the input_line
        patterns.reverse(); // Reverse the patterns
    }

    println!("{:?}", patterns);
    println!("{:?}", input_line);

    'input_iter: for i in 0..input_line.len() {
        let input = &input_line[i..];
        let mut iter = input.chars();
        for pattern in patterns.iter() {
            match pattern {
                Pattern::End => {
                    if i != 0 {
                        continue 'input_iter;
                    }
                }
                Pattern::Start => {
                    if i != 0 {
                        continue 'input_iter;
                    }
                }
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
                Pattern::End => {
                    if iter.next().is_none() {
                        continue 'input_iter;
                    }
                }
            }
        }
        return true;
    }
    return false;
}

pub fn build_group_pattern(iter: &mut Chars) -> (bool, String) {
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

pub fn build_patterns(pattern: &str) -> Vec<Pattern> {
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
            '^' => Pattern::Start,
            '$' => Pattern::End,
            l => Pattern::Literal(l),
        })
    }
    patterns
}
