mod day1;
mod day2;
mod day3;
mod day4;
mod day4_1;
mod day4_2;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day16;
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

        assert_eq!(
            day1::process_puzzle2("The number three is before eight in this line"),
            38
        );

        assert_eq!(day1::process_puzzle2("FourfiveSix"), 46);
        assert_eq!(day1::process_puzzle2("ONEtwoTHREE"), 13);

        assert_eq!(
            day1::process_puzzle2(
                "This is a test string with one number at the start and two at the end"
            ),
            12
        );
    }
}



fn main() {
    if 0 != 0 {
        day1::puzzle1();
        day1::puzzle2();
        day2::play();
        day3::play();
        day4::play();
        day4_1::play();
        day5::play();
        day6::play();
        day7::play();
        day9::play();
        day10::play();
        day11::play();

    }

    day16::play();
}

    
