fn match_pattern(input_line: &str, pattern: &str) -> bool {
    let mut input_iter = input_line.chars().peekable();
    let mut pattern_iter = pattern.chars().peekable();
    let mut backreferences = Vec::new();

    while let Some(&p) = pattern_iter.peek() {
        match p {
            '\\' => {
                pattern_iter.next(); // Consume the backslash
                if let Some(&next_p) = pattern_iter.peek() {
                    match next_p {
                        'w' => {
                            pattern_iter.next(); // Consume 'w'
                            if let Some(&c) = input_iter.peek() {
                                if !c.is_alphanumeric() {
                                    return false;
                                }
                                input_iter.next(); // Consume the alphanumeric character
                            } else {
                                return false;
                            }
                        }
                        'd' => {
                            pattern_iter.next(); // Consume 'd'
                            if let Some(&c) = input_iter.peek() {
                                if !c.is_digit(10) {
                                    return false;
                                }
                                input_iter.next(); // Consume the digit
                            } else {
                                return false;
                            }
                        }
                        _ => return false,
                    }
                }
            }
            '(' => {
                pattern_iter.next(); // Consume '('
                let mut backref = String::new();
                while let Some(&c) = pattern_iter.peek() {
                    if c == ')' {
                        break;
                    }
                    backref.push(c);
                    pattern_iter.next();
                }
                pattern_iter.next(); // Consume ')'
                let mut matched_segment = String::new();
                for _ in 0..backref.len() {
                    if let Some(&c) = input_iter.peek() {
                        matched_segment.push(c);
                        input_iter.next();
                    } else {
                        return false;
                    }
                }
                backreferences.push(matched_segment);
            }
            ' ' => {
                pattern_iter.next(); // Consume space
                if let Some(&c) = input_iter.peek() {
                    if c != ' ' {
                        return false;
                    }
                    input_iter.next(); // Consume space
                } else {
                    return false;
                }
            }
            _ => {
                if let Some(&c) = input_iter.peek() {
                    if c != p {
                        return false;
                    }
                    input_iter.next(); // Consume the character
                    pattern_iter.next(); // Consume the character
                } else {
                    return false;
                }
            }
        }
    }

    true
}

fn main() {
    let input = "grep 101 is doing grep 101 times again and again";
    let pattern = "(\\w\\w\\w\\w \\d\\d\\d) is doing \\1 times (again) and \\2";
    let result = match_pattern(input, pattern);
    println!("Match result: {}", result);
}
