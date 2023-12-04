

#[cfg(test)]
mod tests {
    use super::*; // Import the necessary functions from the parent module

    #[test]
    fn test_for_day1_puzzle2() {
        assert_eq!(day1::process_puzzle2("two1nine"), 29);
        assert_eq!(day1::process_puzzle2("eightwothree"), 83);
        
        assert_eq!(day1::process_puzzle2("1two"), 12);
        assert_eq!(day1::process_puzzle2("three4"), 34);

        assert_eq!(day1::process_puzzle2("onetwo"), 12);
        assert_eq!(day1::process_puzzle2("threefour"), 34);

        assert_eq!(day1::process_puzzle2("1two3"), 13);
        assert_eq!(day1::process_puzzle2("four5six"), 46);

        assert_eq!(day1::process_puzzle2("two2two"), 22);
        assert_eq!(day1::process_puzzle2("one1one"), 11);

        assert_eq!(day1::process_puzzle2("twothreefourfive"), 25);
        assert_eq!(day1::process_puzzle2("sixseveneightnine"), 69);

        assert_eq!(day1::process_puzzle2("abcdef"), 0);

        assert_eq!(day1::process_puzzle2(""), 0);

        assert_eq!(day1::process_puzzle2("one"), 11);

        assert_eq!(day1::process_puzzle2("2xthree!"), 23);

        assert_eq!(day1::process_puzzle2("hellothreefourgoodbye"), 34);

        assert_eq!(day1::process_puzzle2("onehelloonetwo"), 12);

        assert_eq!(day1::process_puzzle2("1seven2eight3"), 13);

        assert_eq!(day1::process_puzzle2("fivenine"), 59);
        
        assert_eq!(day1::process_puzzle2("123four567eight"), 18);

        assert_eq!(day1::process_puzzle2("TwoThree"), 23);

        assert_eq!(day1::process_puzzle2("throneeights"), 38);

        assert_eq!(day1::process_puzzle2("three2five1seven"), 37);

        assert_eq!(day1::process_puzzle2("!two@3#four$"), 24);

        assert_eq!(day1::process_puzzle2("start1middlesevenend"), 17);

        assert_eq!(day1::process_puzzle2("12345six7890"), 60);

        assert_eq!(day1::process_puzzle2("fourfourfourfour"), 44);

        assert_eq!(day1::process_puzzle2("eninoht"), 19);

        assert_eq!(day1::process_puzzle2("2abc3defonehijklmn5"), 25);

        assert_eq!(day1::process_puzzle2("sev#en@8two*4"), 74);

        assert_eq!(day1::process_puzzle2("The number three is before eight in this line"), 38);

        assert_eq!(day1::process_puzzle2("FourfiveSix"), 46);
        assert_eq!(day1::process_puzzle2("ONEtwoTHREE"), 13);

        assert_eq!(day1::process_puzzle2("This is a test string with one number at the start and two at the end"), 12);

    }
}

mod day4 {
    use std::fs::File;
    use std::io::Read;    

    fn process_input_lines( haystack: &str ) -> i32 {
        haystack
            .lines()
            .map(|line| process_input_line(line))
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn process_input_line( haystack: &str ) -> i32 {

        0
    }

    pub fn puzzle1() {
        let mut contents = String::new();
    
        match File::open("data/p2_input.txt") {
            Ok(mut file) => {
                match file.read_to_string(&mut contents) {
                    Ok(_) => println!("Puzzle # 1.1: {}", process_input_lines(contents)),
                    Err(e) => println!("Error reading file: {}", e),
                }
            },
            Err(e) => println!("Error opening file: {}", e),
        }
    }

}
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

        //println!("haystack {}", haystack);
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
        //println!("Result {}", result);
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

    pub fn process_puzzle2( haystack: &str ) -> i32 {
        let temp = replace_earliest_and_last_match_in_each_line(&haystack);                       
        return sum_of_first_and_last_digits_in_each_line(temp);
    }

    pub fn puzzle2() {
        let mut contents = String::new();
        let mut ret = -1;
        
        match File::open("data/p1_input.txt") {
            Ok(mut file) => {
                match file.read_to_string(&mut contents) {
                    Ok(_) => {
                        ret = process_puzzle2(&contents);
                        println!("Puzzle # 2.1: {}", ret)

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


