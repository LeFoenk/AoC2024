use std::fs;

fn main() {
    let file_path = "input.txt";

    let content = fs::read_to_string(file_path).unwrap();

    let mut rules: Vec<Rule> = vec![];
    let mut rules_done = false;
    let mut updates = vec![];
    for line in content.lines() {
        if rules_done == false {
            if line.is_empty() {
                rules_done = true;
                continue;
            }

            let mut iter = line.chars();
            let mut num_str = String::new();
            while let Some(characer) = iter.next() {
                if characer == '|' {
                    break;
                }
                num_str.push(characer);
            }

            let first = num_str.parse::<i32>().unwrap();
            num_str.clear();
            while let Some(character) = iter.next() {
                num_str.push(character);
            }

            let second = num_str.parse::<i32>().unwrap();

            rules.push(Rule { first, second });
        } else {
            let update: Vec<i32> = line
                .split_terminator(',')
                .map(|str| str.parse::<i32>().unwrap())
                .collect();

            updates.push(update);
        }
    }

    let mut result = 0;

    let mut safe = true;
    for update in updates {
        for rule in &rules {
            if let Some(index_1) = update.iter().position(|n| *n == rule.first) {
                if let Some(index_2) = update.iter().position(|n| *n == rule.second) {
                    if index_2 < index_1 {
                        safe = false;
                        break;
                    }
                }
            }
        }

        if safe == true {
            let middle = update[update.len() / 2];
            result += middle;
        }
        safe = true;
    }

    println!("Part 1: {}", result);
}

struct Rule {
    first: i32,
    second: i32,
}
