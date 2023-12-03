use std::collections::HashMap;

pub fn puzzle(input: &str) -> String {
    let lines = input.lines();
    let calibration_values = lines.map(recover_calibration_value_3);

    let answer = calibration_values.fold(0, |accumulator: u32, value: String| {
        accumulator + value.parse().unwrap_or(0)
    });
    println!("The answer is {}", answer);
    format!("{}", answer)
}

fn recover_calibration_value_3(line: &str) -> String {
    let first = get_first_digit(line);
    let last = get_last_digit(line);
    format!("{}{}", first, last)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_recover_calibration_digits_at_start_and_end() {
        let line = "1abc2";
        let expected = "12";

        let result = recover_calibration_value_3(&line);
        assert_eq!(expected, result);
    }

    #[test]
    fn part1_recover_calibration_digits_from_middle() {
        let line = "pqr3stu8vwx";
        let expected = "38";

        let result = recover_calibration_value_3(&line);
        assert_eq!(expected, result);
    }

    #[test]
    fn part1_recover_calibration_digits_extra_digits_in_middle() {
        let line = "a1b2c3d4e5f";
        let expected = "15";

        let result = recover_calibration_value_3(&line);
        assert_eq!(expected, result);
    }

    #[test]
    fn part1_recover_calibration_digits_one_digit() {
        let line = "treb7uchet";
        let expected = "77";

        let result = recover_calibration_value_3(&line);
        assert_eq!(expected, result);
    }

    #[test]
    fn part1_summed_calibration_values() {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        assert_eq!("142", puzzle(input));
    }

    #[test]
   fn part2_recover_calibration_digits_1() {
        assert_eq!("29", recover_calibration_value_3("two1nine"));
   }

    #[test]
    fn part2_recover_calibration_digits_2() {
        assert_eq!("83", recover_calibration_value_3("eightwothree"));
    }

    #[test]
    fn part2_recover_calibration_digits_3() {
        assert_eq!("13", recover_calibration_value_3("abcone2threexyz"));
    }

    #[test]
    fn part2_recover_calibration_digits_4() {
        assert_eq!("24", recover_calibration_value_3("xtwone3four"));
    }

    #[test]
    fn part2_recover_calibration_digits_5() {
        assert_eq!("42", recover_calibration_value_3("4nineeightseven2"));
    }

    #[test]
    fn part2_recover_calibration_digits_6() {
        assert_eq!("14", recover_calibration_value_3("zoneight234"));
    }

    #[test]
    fn part2_recover_calibration_digits_7() {
        assert_eq!("76", recover_calibration_value_3("7pqrstsixteen"));
    }

    #[test]
    fn part2_summed_calibration_values() {
        let input = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        assert_eq!("281", puzzle(input));
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Tricky one that was not in the examples

    #[test]
    fn part2_tricky() {
        assert_eq!(String::from("28"), recover_calibration_value_3("twoneoneight"));
    }

    #[test]
    fn part2_moar_tricky() {
        assert_eq!(String::from("77"), recover_calibration_value_3("7gt"));
    }

    #[test]
    fn part2_even_more_tricky() {
        assert_eq!(String::from("44"), recover_calibration_value_3("fmbbkvthdcdmcjxzclk42six4"));
    }

    ////////////////////////////////////////////////////////////////////////////////

    ////////////////////////////////////////////////////////////////////////////////
    // New implementation

    // get first number word or digit
    #[test]
    fn first_digit_is_1() {
        assert_eq!(String::from("1"), get_first_digit("xxx1xxx"));
    }

    #[test]
    fn first_digit_is_one() {
        assert_eq!(String::from("1"), get_first_digit("xxxoneightxxx"));
    }

    #[test]
    fn first_digit_is_2() {
        assert_eq!(String::from("2"), get_first_digit("xxx2xxx"));
    }

    #[test]
    fn first_digit_is_two() {
        assert_eq!(String::from("2"), get_first_digit("xxxtwoneightxxx"));
    }

    // get last number word or digit
    #[test]
    fn last_digit_is_1() {
        assert_eq!(String::from("1"), get_last_digit("xxx1xxx"));
    }

    #[test]
    fn last_digit_is_one() {
        assert_eq!(String::from("1"), get_last_digit("xxxtwonexxx"));
    }

    #[test]
    fn last_digit_is_2() {
        assert_eq!(String::from("2"), get_last_digit("xxx2xxx"));
    }

    #[test]
    fn last_digit_is_two() {
        assert_eq!(String::from("2"), get_last_digit("xxxeightwoxxx"));
    }

    #[test]
    fn last_digit_is_four() {
        assert_eq!(String::from("4"), get_last_digit("fmbbkvthdcdmcjxzclk42six4"))
    }

}

fn get_candidates() -> HashMap<String, String> {
    let mut digit_numbers_and_words = HashMap::new();
    digit_numbers_and_words.insert(String::from("1"), String::from("1"));
    digit_numbers_and_words.insert(String::from("one"), String::from("1"));
    digit_numbers_and_words.insert(String::from("2"), String::from("2"));
    digit_numbers_and_words.insert(String::from("two"), String::from("2"));
    digit_numbers_and_words.insert(String::from("3"), String::from("3"));
    digit_numbers_and_words.insert(String::from("three"), String::from("3"));
    digit_numbers_and_words.insert(String::from("4"), String::from("4"));
    digit_numbers_and_words.insert(String::from("four"), String::from("4"));
    digit_numbers_and_words.insert(String::from("5"), String::from("5"));
    digit_numbers_and_words.insert(String::from("five"), String::from("5"));
    digit_numbers_and_words.insert(String::from("6"), String::from("6"));
    digit_numbers_and_words.insert(String::from("six"), String::from("6"));
    digit_numbers_and_words.insert(String::from("7"), String::from("7"));
    digit_numbers_and_words.insert(String::from("seven"), String::from("7"));
    digit_numbers_and_words.insert(String::from("8"), String::from("8"));
    digit_numbers_and_words.insert(String::from("eight"), String::from("8"));
    digit_numbers_and_words.insert(String::from("9"), String::from("9"));
    digit_numbers_and_words.insert(String::from("nine"), String::from("9"));

    digit_numbers_and_words
}

fn get_first_digit(line: &str) -> String {
    let digit_numbers_and_words = get_candidates();
    let mut min = line.len();
    let mut best_candidate: Option<String> = None;
    for candidate in digit_numbers_and_words.keys() {
        if let Some(offset) = line.find(candidate) {
            if offset < min {
                min = offset;
                best_candidate = Some(candidate.to_string());
            }
        }
    }
    if let Some(bc) = best_candidate {
        digit_numbers_and_words.get(&bc).unwrap().clone()
    } else {
        println!("WARNING: line {} unlikely handled correctly!", line);
        String::from("0")
    }
}


fn get_last_digit(line: &str) -> String {
    let digit_numbers_and_words = get_candidates();
    let mut max = 0;
    let mut best_candidate: Option<String> = None;
    for candidate in digit_numbers_and_words.keys() {
        if let Some(offset) = line.rfind(candidate) {
            if offset >= max {
                max = offset;
                best_candidate = Some(candidate.to_string());
            }
        }
    }
    if let Some(bc) = best_candidate {
        digit_numbers_and_words.get(&bc).unwrap().clone()
    } else {
        println!("WARNING: line {} unlikely handled correctly!", line);
        String::from("0")
    }
}
