use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn extract_and_combine_numbers(input: &str) -> Option<u32> {
    let first_number = input.chars().find(|c| c.is_digit(10));
    let last_number = input.chars().rev().find(|c| c.is_digit(10));

    match (first_number, last_number) {
        (Some(first), Some(last)) => {
            let combined_number: String = format!("{}{}", first, last);
            Some(combined_number.parse::<u32>().unwrap_or(0))
        }
        _ => None,
    }
}

fn read_file_lines(file_path: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Collect lines into a Vec<String>
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    Ok(lines)
}

pub fn day1() {
    let file_path = "input/input1.txt";

    match read_file_lines(file_path) {
        Ok(lines) => {
            let number: u32 = lines.iter().map(|line| extract_and_combine_numbers(&line)).filter_map(|number| number).sum();
            println!("{}", number);
        }
        Err(err) => eprintln!("Error reading file: {}", err),
    }
}