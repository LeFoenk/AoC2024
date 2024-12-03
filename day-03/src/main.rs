use std::fs;

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

    Ok(())
}
