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

    println!("Number of safe reports: {}", safe_reports);

    Ok(())
}
