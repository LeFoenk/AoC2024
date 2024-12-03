use std::{fs, str::Chars};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_name = "input.txt";

    let content = fs::read_to_string(file_name)?;

    let mut start = 0;
    let delimiter = "mul(";
    let mut result = 0;
    while let Some(offset) = content[start..].find(delimiter) {
        start = start + offset + delimiter.len();

        let mut number_1 = String::new();
        while let Some(character) = content[start..].chars().next() {
            if character.is_numeric() {
                number_1.push(character);
            } else {
                break;
            }
            start += 1;
        }

        if let Some(character) = content.chars().nth(start) {
            if character != ',' {
                continue;
            }
            start += 1;
        } else {
            continue;
        }

        let mut number_2 = String::new();
        while let Some(character) = content[start..].chars().next() {
            if character.is_numeric() {
                number_2.push(character);
            } else {
                break;
            }
            start += 1;
        }

        if let Some(character) = content.chars().nth(start) {
            if character != ')' {
                continue;
            }
            start += 1;
        } else {
            continue;
        }

        let number_1 = number_1.parse::<i32>()?;
        let number_2 = number_2.parse::<i32>()?;

        result += number_1 * number_2;
    }

    println!("Part 1: {}", result);

    let mut scanner = MemoryScanner {
        content: content.chars(),
        is_active: true,
        result: 0,
    };
    result = scanner.parse();

    println!("Part 2: {}", result);

    Ok(())
}

struct MemoryScanner<'a> {
    content: Chars<'a>,
    is_active: bool,
    result: i32,
}

impl MemoryScanner<'_> {
    fn parse(&mut self) -> i32 {
        while let Some(character) = self.content.next() {
            match character {
                'd' => self.activate(),
                'm' => {
                    if self.is_active {
                        self.multiply();
                    }
                }
                _ => {}
            }
        }
        self.result
    }

    fn multiply(&mut self) {
        let mut character = self.content.next();
        if character != Some('u') {
            return;
        }

        character = self.content.next();
        if character != Some('l') {
            return;
        }

        character = self.content.next();
        if character != Some('(') {
            return;
        }

        let mut num_1 = String::new();
        while let Some(num_char) = self.content.next() {
            match num_char {
                '0'..='9' => num_1.push(num_char),
                ',' => break,
                _ => return,
            }
        }

        if num_1.is_empty() {
            return;
        }

        let mut num_2 = String::new();
        while let Some(num_char) = self.content.next() {
            match num_char {
                '0'..='9' => num_2.push(num_char),
                ')' => break,
                _ => return,
            }
        }

        println!("left: {}, right: {}, result: {}", num_1, num_2, self.result);

        let num_1 = num_1.parse::<i32>().unwrap();
        let num_2 = num_2.parse::<i32>().unwrap();
        self.result += num_1 * num_2;
    }

    fn activate(&mut self) {
        if let Some(character) = self.content.next() {
            if character == 'o' {
                if let Some(char_2) = self.content.next() {
                    if char_2 == '(' {
                        if let Some(char_3) = self.content.next() {
                            if char_3 == ')' {
                                self.is_active = true;
                                return;
                            }
                        }
                    } else if char_2 == 'n' {
                        if let Some(char_3) = self.content.next() {
                            if char_3 != '\'' {
                                return;
                            }

                            if let Some(char_4) = self.content.next() {
                                if char_4 != 't' {
                                    return;
                                }

                                if let Some(char_5) = self.content.next() {
                                    if char_5 != '(' {
                                        return;
                                    }

                                    if let Some(char_6) = self.content.next() {
                                        if char_6 == ')' {
                                            self.is_active = false;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
