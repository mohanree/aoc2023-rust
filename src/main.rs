
mod day1 {
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::{Read};    

    fn replace_earliest_and_last_match_in_each_line(haystack: &str) -> String {
        haystack
            .lines()
            .map(|line| replace_first_and_last_matches(line))
            .collect::<Vec<_>>()
            .join("\n")
    }


    fn replace_first_and_last_matches(haystack: &str) -> String {
        let needles: HashMap<&str, &str> = [
            ("one", "1"), ("two", "2"), ("three", "3"),
            ("four", "4"), ("five", "5"), ("six", "6"),
            ("seven", "7"), ("eight", "8"), ("nine", "9"),
        ].iter().cloned().collect();

        let mut first_match: Option<(usize, &str, usize)> = None;
        let mut last_match: Option<(usize, &str, usize)> = None;

        println!("haystack {}", haystack);
        for (needle, replacement) in &needles {
            if let Some(index) = haystack.find(needle) {
                if first_match.is_none() || index < first_match.unwrap().0 {
                    first_match = Some((index, replacement, needle.len()));
                }
            }
            if let Some(index) = haystack.rfind(needle) {
                if last_match.is_none() || index > last_match.unwrap().0 {
                    last_match = Some((index, replacement, needle.len()));
                }
            }
        }

        let mut result = haystack.to_string();
        if let Some((index, replacement, len)) = last_match {
            result.replace_range(index..index + len, replacement);
        }
        if let Some((index, replacement, len)) = first_match {
            if first_match != last_match {
                result.replace_range(index..index + len, replacement);
            }
        }
        println!("Result {}", result);
        result
    }

    fn sum_of_first_and_last_digits_in_each_line(contents: String) -> i32 {
        let mut sum = 0;
        for line in contents.lines() {
            let numeric_chars: Vec<char> = line.chars().filter(|c| c.is_numeric()).collect();
    
            if let (Some(first_char), Some(last_char)) = (numeric_chars.first(), numeric_chars.last()) {
                let first_digit = first_char.to_digit(10).unwrap() as i32;
                let last_digit = last_char.to_digit(10).unwrap() as i32;
                sum += first_digit * 10 + last_digit;
            }
        }
        sum
    }
    
    pub fn puzzle1() {
        let mut contents = String::new();
    
        match File::open("data/p1_input.txt") {
            Ok(mut file) => {
                match file.read_to_string(&mut contents) {
                    Ok(_) => println!("Puzzle # 1.1: {}", sum_of_first_and_last_digits_in_each_line(contents)),
                    Err(e) => println!("Error reading file: {}", e),
                }
            },
            Err(e) => println!("Error opening file: {}", e),
        }
    }

    pub fn puzzle2() {
        let mut contents = String::new();
    
        match File::open("data/p1_input.txt") {
            Ok(mut file) => {
                match file.read_to_string(&mut contents) {
                    Ok(_) => {
                        let temp = replace_earliest_and_last_match_in_each_line(&contents);                       
                        println!("Puzzle # 2.1: {}", sum_of_first_and_last_digits_in_each_line(temp))

                    },
                    Err(e) => println!("Error reading file: {}", e),
                }
            },
            Err(e) => println!("Error opening file: {}", e),
        }
    }


}

fn main() {
    day1::puzzle1();
    day1::puzzle2();
}


