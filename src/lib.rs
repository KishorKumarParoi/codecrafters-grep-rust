// use std::str::Chars;

// #[derive(Debug, PartialEq)] // Add PartialEq here
// pub enum Pattern {
//     Literal(char),
//     Digit,
//     Alphanumeric,
//     Group(bool, String),
//     Start,
//     End,
//     OneOrMore {
//         min: u32,
//         pattern: Box<Pattern>,
//         max: Option<u32>,
//     },
//     ZeroOrOnce {
//         pattern: Box<Pattern>,
//     },
//     Wildcard,
//     MatchMoreWord,
//     SingleBackreference,
// }

// pub fn match_literal(chars: &mut Chars, literal: char) -> bool {
//     chars.next().map_or(false, |c| c == literal)
// }

// pub fn match_digit(chars: &mut Chars) -> bool {
//     chars.next().map_or(false, |c| c.is_digit(10))
// }

// pub fn match_alphanumeric(chars: &mut Chars) -> bool {
//     chars.next().map_or(false, |c| c.is_alphanumeric())
// }

// pub fn match_group(chars: &mut Chars, group: &str) -> bool {
//     chars.next().map_or(false, |c| group.contains(c))
// }

// pub fn make_flat_string(input_line: &str) -> String {
//     input_line.chars().filter(|&c| c != '?').collect()
// }

// pub fn wildcard_string(input_line: &str) -> String {
//     input_line.chars().filter(|&c| c != '.').collect()
// }

// /*
// echo -n "grep 101 is doing grep 101 times agin and again" |
// ./your_program.sh -E "(\w\w\w\w \d\d\d) is doing \1 times (again) and again"
// */
// fn match_backreference_pattern(input_line: &str, pattern: &str) -> bool {
//     println!("Input Line: {:?}", input_line);
//     println!("Pattern: {:?}", pattern);

//     let mut backreferences = Vec::new();
//     let mut full_string = String::new();

//     'input_iter: for i in 0..input_line.len() {
//         let mut pattern_iter = pattern.chars().peekable();
//         let input = &input_line[i..];
//         let mut input_iter = input.chars().peekable();

//         while let Some(&p) = pattern_iter.peek() {
//             match p {
//                 '(' => {
//                     pattern_iter.next(); // Consume '('
//                     let mut backref = String::new();

//                     while let Some(&cc) = pattern_iter.peek() {
//                         println!("cc: {:?}", cc);
//                         if cc == ')' {
//                             backreferences.push(backref);
//                             break;
//                         }

//                         match cc {
//                             '\\' => {
//                                 pattern_iter.next(); // Consume the backslash
//                                 if let Some(&next_p) = pattern_iter.peek() {
//                                     match next_p {
//                                         'w' => {
//                                             pattern_iter.next(); // Consume 'w'
//                                             if let Some(&c) = input_iter.peek() {
//                                                 if !c.is_alphanumeric() {
//                                                     continue 'input_iter;
//                                                 }
//                                                 full_string.push(c);
//                                                 backref.push(c);
//                                                 input_iter.next(); // Consume the alphanumeric character
//                                             } else {
//                                                 continue 'input_iter;
//                                             }
//                                         }
//                                         'd' => {
//                                             pattern_iter.next(); // Consume 'd'
//                                             if let Some(&c) = input_iter.peek() {
//                                                 if !c.is_digit(10) {
//                                                     continue 'input_iter;
//                                                 }
//                                                 full_string.push(c);
//                                                 backref.push(c);
//                                                 input_iter.next(); // Consume the digit
//                                             } else {
//                                                 continue 'input_iter;
//                                             }
//                                         }
//                                         '+' => {
//                                             pattern_iter.next(); // Consume '+'
//                                             while let Some(&c) = input_iter.peek() {
//                                                 full_string.push(c);
//                                                 backref.push(c);
//                                                 input_iter.next();

//                                                 if c == ' ' {
//                                                     break;
//                                                 }
//                                             }
//                                         } // _ => {
//                                           //     pattern_iter.next(); // Consume space
//                                           //     if let Some(&c) = input_iter.peek() {
//                                           //         if c != pattern_iter.peek() {
//                                           //             continue 'input_iter;
//                                           //         }
//                                           //         full_string.push(c);
//                                           //         backref.push(c);
//                                           //         input_iter.next();
//                                           //     } else {
//                                           //         continue 'input_iter;
//                                           //     }
//                                           // }
//                                     }
//                                 } else {
//                                     continue 'input_iter;
//                                 }
//                             }
//                             '.' => {
//                                 pattern_iter.next(); // Consume '.'
//                                 if let Some(&c) = input_iter.peek() {
//                                     full_string.push(c);
//                                     backref.push(c);
//                                     input_iter.next();
//                                 }
//                             }
//                             ' ' => {
//                                 pattern_iter.next(); // Consume space
//                                 if let Some(&c) = input_iter.peek() {
//                                     if c != ' ' {
//                                         continue 'input_iter;
//                                     }
//                                     full_string.push(c);
//                                     backref.push(c);
//                                     input_iter.next();
//                                 } else {
//                                     continue 'input_iter;
//                                 }
//                             }
//                             _ => {
//                                 if let Some(&c) = input_iter.peek() {
//                                     full_string.push(c);
//                                     backref.push(c);
//                                     input_iter.next();
//                                 }
//                             }
//                         }
//                     }

//                     pattern_iter.next(); // Consume ')'
//                 }
//                 '\\' => {
//                     pattern_iter.next(); // Consume the backslash
//                     if let Some(&next_p) = pattern_iter.peek() {
//                         if next_p.is_digit(10) {
//                             pattern_iter.next(); // Consume the digit
//                             if let Some(index) = next_p.to_digit(10) {
//                                 let index = index as usize - 1;
//                                 if index < backreferences.len() {
//                                     full_string.push_str(&backreferences[index]);
//                                 } else {
//                                     continue 'input_iter; // Index out of bounds
//                                 }
//                             }
//                         }
//                     }
//                 }
//                 _ => {
//                     if let Some(&c) = input_iter.peek() {
//                         full_string.push(c);
//                         input_iter.next();
//                     }
//                 }
//             }
//         }
//     }

//     println!("Full String: {:?}", full_string);
//     println!("Input Line: {:?}", input_line);

//     if input_line.contains(&full_string) {
//         return true;
//     }

//     false
// }

// pub fn match_pattern(input_line: &str, pattern: &str) -> bool {
//     let pattern_copy = pattern.to_string();
//     let flat_str = make_flat_string(pattern);
//     let mut patterns = build_patterns(pattern);

//     let mut input_line = input_line.trim_matches('\n').to_string(); // Make input_line mutable

//     if let Some(Pattern::End) = patterns.last() {
//         input_line = input_line.chars().rev().collect(); // Reverse the input_line
//         patterns.reverse(); // Reverse the patterns
//     }

//     // println!("Patterns -> {:?}", patterns);
//     // println!("input_line -> {:?}", input_line);

//     'input_iter: for i in 0..input_line.len() {
//         let input = &input_line[i..];
//         // println!("input: {}, i: {}", input, i);
//         let mut iter = input.chars();
//         for pattern in &patterns {
//             // println!("input: {}, i: {}, {:?}", input, i, pattern);
//             match pattern {
//                 Pattern::Start | Pattern::End => {
//                     if i != 0 {
//                         continue 'input_iter;
//                     }
//                 }
//                 Pattern::SingleBackreference => {
//                     return match_backreference_pattern(input, &pattern_copy);
//                 }
//                 Pattern::MatchMoreWord => {
//                     let mut word = String::new();
//                     let mut words = Vec::new();
//                     let start_pos = pattern_copy.find('(');
//                     let end_pos = pattern_copy.find(')');
//                     let string = &pattern_copy[start_pos.unwrap() + 1..end_pos.unwrap()];

//                     println!("String: {:?}", string);

//                     for c in string.chars() {
//                         if c == '|' {
//                             if !word.is_empty() {
//                                 words.push(word.clone());
//                                 word.clear();
//                             }
//                         } else {
//                             word.push(c);
//                         }
//                     }
//                     if !word.is_empty() {
//                         words.push(word.clone());
//                     }

//                     println!("Input: {:?}", input);
//                     println!("Words: {:?}", words);

//                     for word in words {
//                         if input.contains(&word) {
//                             return true;
//                         }
//                     }
//                 }
//                 Pattern::Wildcard => {
//                     let first = if let Some(pos) = pattern_copy.find('.') {
//                         &pattern_copy[..pos]
//                     } else {
//                         ""
//                     };
//                     let last = if let Some(pos) = pattern_copy.rfind('.') {
//                         &pattern_copy[pos + 1..]
//                     } else {
//                         ""
//                     };
//                     println!("First: {}, Last: {}", first, last);
//                     if input.contains(first) && input.contains(last) {
//                         return true;
//                     } else {
//                         continue 'input_iter;
//                     }
//                 }
//                 Pattern::ZeroOrOnce { pattern } => {
//                     println!("ZeroOrOnce");
//                     let val = match **pattern {
//                         Pattern::Literal(c) => c,
//                         _ => continue 'input_iter, // Handle other cases if necessary
//                     };
//                     println!("Val: {}", val);
//                     println!("Flat String: {:?}", flat_str);

//                     let without_pattern_char_string =
//                         flat_str.chars().filter(|&c| c != val).collect::<String>();

//                     println!(
//                         "Without Pattern Char String: {:?}",
//                         without_pattern_char_string
//                     );

//                     if input.contains(&without_pattern_char_string) || input.contains(&flat_str) {
//                         return true;
//                     } else {
//                         continue 'input_iter;
//                     }
//                 }
//                 Pattern::OneOrMore {
//                     min: _,
//                     pattern,
//                     max: _,
//                 } => {
//                     // println!("OneOrMore");
//                     let val = match **pattern {
//                         Pattern::Literal(c) => c,
//                         _ => continue 'input_iter, // Handle other cases if necessary
//                     };
//                     // println!("Val: {}", val);
//                     if input.contains(val) {
//                         return true;
//                     } else {
//                         continue 'input_iter;
//                     }
//                 }
//                 Pattern::Literal(l) => {
//                     if !match_literal(&mut iter, *l) {
//                         continue 'input_iter;
//                     }
//                 }
//                 Pattern::Digit => {
//                     if !match_digit(&mut iter) {
//                         continue 'input_iter;
//                     }
//                 }
//                 Pattern::Alphanumeric => {
//                     if !match_alphanumeric(&mut iter) {
//                         continue 'input_iter;
//                     }
//                 }
//                 Pattern::Group(positive, group) => {
//                     if match_group(&mut iter, group) != *positive {
//                         continue 'input_iter;
//                     }
//                 }
//             }
//         }
//         return true;
//     }
//     false
// }

// pub fn build_group_pattern(iter: &mut Chars) -> (bool, String) {
//     let mut group = String::new();
//     let mut positive = true;
//     if iter.clone().next() == Some('^') {
//         positive = false;
//         iter.next();
//     }
//     while let Some(member) = iter.next() {
//         if member == ']' {
//             break;
//         }
//         group.push(member);
//     }
//     (positive, group)
// }

// pub fn build_patterns(pattern: &str) -> Vec<Pattern> {
//     let mut iter = pattern.chars();
//     let mut patterns = Vec::new();
//     while let Some(current) = iter.next() {
//         patterns.push(match current {
//             '\\' => match iter.next() {
//                 Some('d') => Pattern::Digit,
//                 Some('w') => Pattern::Alphanumeric,
//                 Some('\\') => Pattern::Literal('\\'),
//                 _ => panic!("Invalid special character"),
//             },
//             '[' => {
//                 let (positive, group) = build_group_pattern(&mut iter);
//                 Pattern::Group(positive, group)
//             }
//             '^' => Pattern::Start,
//             '$' => Pattern::End,
//             '+' => {
//                 let last_pattern = patterns.pop().unwrap();
//                 // println!("Last Pattern: {:?}", last_pattern);
//                 patterns.push({
//                     let min = 1;
//                     let max = None;
//                     Pattern::OneOrMore {
//                         min,
//                         max,
//                         pattern: Box::new(last_pattern),
//                     }
//                 });
//                 continue;
//             }
//             '?' => {
//                 let last_pattern = patterns.pop().unwrap();
//                 patterns.push({
//                     let pattern = Box::new(last_pattern);
//                     Pattern::ZeroOrOnce { pattern }
//                 });
//                 continue;
//             }
//             '(' => {
//                 if pattern.ends_with(')') {
//                     Pattern::MatchMoreWord
//                 } else {
//                     patterns.push(Pattern::SingleBackreference);
//                     break;
//                 }
//             }
//             '.' => Pattern::Wildcard,
//             l => Pattern::Literal(l),
//         });
//     }
//     patterns
// }
