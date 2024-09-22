use std::str::Chars;

#[derive(Debug, PartialEq)] // Add PartialEq here
pub enum Pattern {
    Literal(char),
    Digit,
    Alphanumeric,
    Group(bool, String),
    Start,
    End,
    OneOrMore(Box<Pattern>),
}

pub fn match_literal(chars: &mut Chars, literal: char) -> bool {
    chars.next().map_or(false, |c| c == literal)
}

pub fn match_digit(chars: &mut Chars) -> bool {
    chars.next().map_or(false, |c| c.is_digit(10))
}

pub fn match_alphanumeric(chars: &mut Chars) -> bool {
    chars.next().map_or(false, |c| c.is_alphanumeric())
}

pub fn match_group(chars: &mut Chars, group: &str) -> bool {
    chars.next().map_or(false, |c| group.contains(c))
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
        for pattern in &patterns {
            match pattern {
                Pattern::Start | Pattern::End => {
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
                Pattern::OneOrMore(p) => {
                    let val = match **p {
                        Pattern::Literal(c) => c,
                        _ => panic!("Invalid pattern"),
                    };
                    if input_line.clone().contains(val) {
                        // println!("Contains: {}", val);
                        return true;
                    } else {
                        continue 'input_iter;
                    }
                }
            }
        }
        return true;
    }
    false
}

pub fn build_group_pattern(iter: &mut Chars) -> (bool, String) {
    let mut group = String::new();
    let mut positive = true;
    if iter.clone().next() == Some('^') {
        positive = false;
        iter.next();
    }
    while let Some(member) = iter.next() {
        if member == ']' {
            break;
        }
        group.push(member);
    }
    (positive, group)
}

pub fn build_patterns(pattern: &str) -> Vec<Pattern> {
    let mut iter = pattern.chars();
    let mut patterns = Vec::new();
    while let Some(current) = iter.next() {
        patterns.push(match current {
            '\\' => match iter.next() {
                Some('d') => Pattern::Digit,
                Some('w') => Pattern::Alphanumeric,
                Some('\\') => Pattern::Literal('\\'),
                _ => panic!("Invalid special character"),
            },
            '[' => {
                let (positive, group) = build_group_pattern(&mut iter);
                Pattern::Group(positive, group)
            }
            '^' => Pattern::Start,
            '$' => Pattern::End,
            '+' => {
                let last_pattern = patterns.pop().unwrap();
                patterns.push(Pattern::OneOrMore(Box::new(last_pattern)));
                continue;
            }
            l => Pattern::Literal(l),
        });
    }
    patterns
}
