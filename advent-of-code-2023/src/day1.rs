use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn word_to_number(input: String) -> String {
    let word_number = vec![("one", "o1e"),("two", "t2o"),("three", "th3ee"),("four", "fo4r"),("five", "f5ve"),("six", "s6x"),("seven", "se7en"),("eight", "ei8ht"),("nine", "ni9e")];
    let mut new = input.clone();

    for (word, num) in word_number {
        new = new.replace(word, num);
    }

    /*for (word, num) in word_number {
        new = new.replace(word, num);
    }*/

    /*let mut indexs: Vec<((&str, &str), usize)> = word_number.iter().map(|(word, num)| {
        //let index = input.find(word);
        match input.find(word) {
            Some(index) => Some(((*word, *num), index)),
            None => None
        }
    })
    .filter_map(|tuple| tuple)
    .collect();

    indexs.sort_by(|a, b| a.1.cmp(&b.1));

    if let Some(first) = indexs.get(0) {
        new = new.replace(first.0.0, first.0.1);
    }

    if let Some(last) = indexs.last() {
        new = new.replace(last.0.0, last.0.1);
    }*/


    //new = new.replace(indexs[0], num);

    /*for (word, num) in word_number {
        let index = index.find(word);
    }*/

    //println!("{:?}", indexs);

    //println!("{}, {}",input, new);

    return new;
}

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

fn part1() {
    let file_path = "input/input1.txt";

    match read_file_lines(file_path) {
        Ok(lines) => {
            let number: u32 = lines.iter().map(|line| extract_and_combine_numbers(&line)).filter_map(|number| number).sum();
            println!("{}", number);
        }
        Err(err) => eprintln!("Error reading file: {}", err),
    }
}

fn part2() {
    let file_path = "input/input1.txt";
    match read_file_lines(file_path) {
        Ok(lines) => {
            let number: u32 = lines
            .iter()
            .map(|line| extract_and_combine_numbers(&word_to_number(line.to_string())))
            .filter_map(|number| number)
            .sum();

            println!("{}", number);
            //word_to_number("zhlzhrkljonephkgdzsnlglmxvprlh6n".to_string());
            /*for line in lines {
                word_to_number(line.to_string());
            }*/
//            lines.iter().map(|line| word_to_number(line.to_string())).collect();
        }
        Err(err) => eprintln!("Error reading file: {}", err),
    }

}

pub fn day1() {
    part1();
    part2();
}