use std::env;
use std::io;
use std::process;

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    // Create peekable iterators for the input line and the pattern
    let mut input_chars = input_line.chars().peekable();
    let mut pattern_chars = pattern.chars().peekable();

    while let Some(&p) = pattern_chars.peek() {
        match p {
            '\\' => {
                pattern_chars.next(); // Consume the backslash
                if let Some(&next_p) = pattern_chars.peek() {
                    match next_p {
                        'd' => {
                            pattern_chars.next(); // Consume 'd'
                            if let Some(&c) = input_chars.peek() {
                                if !c.is_digit(10) {
                                    return false;
                                }
                                input_chars.next(); // Consume the digit
                            } else {
                                return false;
                            }
                        }
                        'w' => {
                            pattern_chars.next(); // Consume 'w'
                            if let Some(&c) = input_chars.peek() {
                                if !c.is_alphanumeric() {
                                    return false;
                                }
                                input_chars.next(); // Consume the alphanumeric character
                            } else {
                                return false;
                            }
                        }
                        _ => return false,
                    }
                } else {
                    return false;
                }
            }
            '[' => {
                pattern_chars.next(); // Consume '['
                let mut char_set = Vec::new();
                let mut negated = false;
                if let Some(&next_p) = pattern_chars.peek() {
                    if next_p == '^' {
                        negated = true;
                        pattern_chars.next(); // Consume '^'
                    }
                }
                while let Some(&c) = pattern_chars.peek() {
                    if c == ']' {
                        break;
                    }
                    char_set.push(c);
                    pattern_chars.next(); // Consume the character
                }
                if pattern_chars.peek() == Some(&']') {
                    pattern_chars.next(); // Consume ']'
                } else {
                    return false;
                }
                if let Some(&c) = input_chars.peek() {
                    if negated {
                        if char_set.contains(&c) {
                            return false;
                        }
                    } else {
                        if !char_set.contains(&c) {
                            return false;
                        }
                    }
                    input_chars.next(); // Consume the character
                } else {
                    return false;
                }
            }
            _ => {
                if let Some(&c) = input_chars.peek() {
                    if c != p {
                        return false;
                    }
                    input_chars.next(); // Consume the character
                    pattern_chars.next(); // Consume the pattern character
                } else {
                    return false;
                }
            }
        }
    }

    input_chars.peek().is_none()
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
    }

    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    // Uncomment this block to pass the first stage
    if match_pattern(&input_line.trim(), &pattern) {
        println!("{}, {}", &input_line.trim(), &pattern);
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
