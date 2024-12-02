use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let file_path = "input.txt";

    let contents = fs::read_to_string(file_path)?;

    println!("File: {}", contents);

    let mut safe_reports = 0;
    let mut is_safe = true;
    let mut is_descending: Option<bool> = None;
    for line in contents.split_terminator('\n') {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        for pair in numbers.windows(2) {
            if let [l, r] = pair {
                if l == r || i32::abs(l - r) > 3 {
                    is_safe = false;
                    break;
                }
                match is_descending {
                    Some(true) => {
                        if l < r {
                            is_safe = false;
                            break;
                        }
                    }
                    Some(false) => {
                        if l > r {
                            is_safe = false;
                            break;
                        }
                    }
                    None => {
                        is_descending = Some(l > r);
                    }
                }
            }
        }
        if is_safe == true {
            safe_reports += 1;
        }
        is_safe = true;
        is_descending = None;
    }

    println!("Number of safe reports part 1: {}", safe_reports);

    // Part 2

    is_safe = true;
    safe_reports = 0;
    is_descending = None;
    let mut fail_index = 0;
    for line in contents.split_terminator('\n') {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        for (index, pair) in numbers.windows(2).enumerate() {
            if let [l, r] = pair {
                if l == r || i32::abs(l - r) > 3 {
                    is_safe = false;
                    fail_index = index;
                    break;
                }
                match is_descending {
                    Some(true) => {
                        if l < r {
                            is_safe = false;
                            fail_index = index;
                            break;
                        }
                    }
                    Some(false) => {
                        if l > r {
                            is_safe = false;
                            fail_index = index;
                            break;
                        }
                    }
                    None => {
                        is_descending = Some(l > r);
                    }
                }
            }
        }

        let mut numbers_removed_1 = numbers.clone();
        if is_safe == false {
            is_safe = true;
            is_descending = None;
            numbers_removed_1.remove(fail_index);

            for pair in numbers_removed_1.windows(2) {
                if let [l, r] = pair {
                    if l == r || i32::abs(l - r) > 3 {
                        is_safe = false;
                        break;
                    }
                    match is_descending {
                        Some(true) => {
                            if l < r {
                                is_safe = false;
                                break;
                            }
                        }
                        Some(false) => {
                            if l > r {
                                is_safe = false;
                                break;
                            }
                        }
                        None => {
                            is_descending = Some(l > r);
                        }
                    }
                }
            }
        }

        let mut numbers_removed_2 = numbers.clone();
        if is_safe == false && fail_index < numbers_removed_2.len() {
            is_safe = true;
            is_descending = None;
            numbers_removed_2.remove(fail_index + 1);

            for pair in numbers_removed_2.windows(2) {
                if let [l, r] = pair {
                    if l == r || i32::abs(l - r) > 3 {
                        is_safe = false;
                        break;
                    }
                    match is_descending {
                        Some(true) => {
                            if l < r {
                                is_safe = false;
                                break;
                            }
                        }
                        Some(false) => {
                            if l > r {
                                is_safe = false;
                                break;
                            }
                        }
                        None => {
                            is_descending = Some(l > r);
                        }
                    }
                }
            }
        }

        let mut numbers_removed_3 = numbers.clone();
        if is_safe == false && fail_index > 0 {
            is_safe = true;
            is_descending = None;
            numbers_removed_3.remove(fail_index - 1);

            for pair in numbers_removed_3.windows(2) {
                if let [l, r] = pair {
                    if l == r || i32::abs(l - r) > 3 {
                        is_safe = false;
                        break;
                    }
                    match is_descending {
                        Some(true) => {
                            if l < r {
                                is_safe = false;
                                break;
                            }
                        }
                        Some(false) => {
                            if l > r {
                                is_safe = false;
                                break;
                            }
                        }
                        None => {
                            is_descending = Some(l > r);
                        }
                    }
                }
            }
        }

        if is_safe == true {
            safe_reports += 1;
        } else {
            println!(
                "{:?}\n{:?}\n{:?}\n----------------\n",
                numbers, numbers_removed_1, numbers_removed_2
            );
        }
        is_safe = true;
        is_descending = None;
    }
    println!("Number of safe reports part 2: {}", safe_reports);

    Ok(())
}
