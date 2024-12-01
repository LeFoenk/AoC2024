use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "src/input.txt";

    let content = fs::read_to_string(file_path)?;

    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in content.split('\n') {
        match line.split_once(' ') {
            Some((num_left, num_right)) => {
                left.push(num_left.parse::<i32>()?);
                right.push(num_right.trim_start().parse::<i32>()?);
            }
            None => {}
        }
    }

    left.sort();
    right.sort();

    let difference: i32 = left
        .iter()
        .zip(right.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

    println!("Difference: {}", difference);

    // PART 2
    let mut similarity = 0;
    for number in left {
        let count = right.iter().filter(|&&x| x == number).count();
        similarity = similarity + (number * count as i32);
    }

    println!("Similarity: {}", similarity);

    Ok(())
}
