use std::collections::HashMap;

pub fn puzzle(input: &str) -> String {
    let lines = input.lines();
    let calibration_values = lines.map(recover_massaged_calibration_value);

    let answer = calibration_values.fold(0, |accumulator: u32, value: String| {
        accumulator + value.parse().unwrap_or(0)
    });
    println!("The answer is {}", answer);
    format!("{}", answer)
}

fn recover_massaged_calibration_value(line: &str) -> String {
    let massaged_line = replace_words_with_numbers(line);
    println!("massaged line: {}", massaged_line);
    let recovered = recover_calibration_value(&massaged_line);
    println!("recovered: {}", recovered);
    recovered
}

fn replace_words_with_numbers(line: &str) -> String {
    line
        .replace("one", "1")
        .replace("two", "2")
        .replace("three", "3")
        .replace("four", "4")
        .replace("five", "5")
        .replace("six", "6")
        .replace("seven", "7")
        .replace("eight", "8")
        .replace("nine", "9")
}

fn recover_calibration_value(line: &str) -> String {
    let matches: Vec<&str> = line.matches(char::is_numeric).collect();
    format!("{}{}", matches[0], matches[matches.len() - 1])
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test list:
    // 1abc2        12
    // pqr3stu8vwx  38
    // a1b2c3d4e5f  15
    // treb7uchet   77

    // Invariants
    // Always return a string with two characters
    // Both characters match [0-9]

    #[test]
    fn test_digits_at_start_and_end() {
        let line = "1abc2";
        let expected = "12";

        let result = recover_calibration_value(&line);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_digits_in_middle() {
        let line = "pqr3stu8vwx";
        let expected = "38";

        let result = recover_calibration_value(&line);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_extra_digits() {
        let line = "a1b2c3d4e5f";
        let expected = "15";

        let result = recover_calibration_value(&line);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_one_digit() {
        let line = "treb7uchet";
        let expected = "77";

        let result = recover_calibration_value(&line);
        assert_eq!(expected, result);
    }

    #[test]
    fn summed_calibration_values() {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        assert_eq!("142", puzzle(input));
    }

    #[test]
    fn test_replace_word_with_number_1() {
        let line = "two1nine";
        let expected = "219";

        let result = replace_words_with_numbers(line);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_replace_word_with_number_2() {
        let line = "eightwothree";
        let expected = "8wo3";

        let result = replace_words_with_numbers(line);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_replace_word_with_number_and_recover_calibration_value() {
        let line = "two1nine";
        let expected = "29";

        let result = replace_words_with_numbers(line);
        assert_eq!(expected, recover_calibration_value(&result));
    }

    // #[test]
    // fn test_recover_massaged_calibration_values() {
    //     assert_eq!("29", recover_massaged_calibration_value("two1nine"));
    //     assert_eq!("83", recover_massaged_calibration_value("eightwothree"));
    //     assert_eq!("13", recover_massaged_calibration_value("abcone2threexyz"));
    //     assert_eq!("24", recover_massaged_calibration_value("xtwone3four"));
    //     assert_eq!("42", recover_massaged_calibration_value("4nineeightseven2"));
    //     assert_eq!("14", recover_massaged_calibration_value("zoneight234"));
    //     assert_eq!("76", recover_massaged_calibration_value("7pqrstsixteen"));
    // }

    fn parse1(line: &str) -> (String, String) {
        let mut number_words = HashMap::new();
        number_words.insert(String::from("eight"), String::from("8"));
        number_words.insert(String::from("three"), String::from("3"));

        let mut max = 0;
        for number_word in number_words.keys() {
            if let Some(offset) = line.find(number_word) {
                if offset == 0 {
                    let token = number_word;
                    let token_value = number_words.get(token).unwrap();
                    let token_length = token.len();
                    return (token_value.clone(), String::from(&line[token_length..line.len()]))
                } else {
                    max = offset;
                }
            }
        }
        if max > 0 {
            let gibberish = String::from(&line[0..max]);
            let rest = String::from(&line[max..line.len()]);
            return (gibberish, rest);
        }
        (String::from(""), String::from(""))
    }

    fn parse_all(line: &str) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        let mut rest = String::from(line);
        while rest.len() > 0 {
            let (token, rest2) = parse1(&rest);
            result.push(token);
            rest = rest2;
        }

        result
    }

    #[test]
    fn parse1_token_number_word_at_start() {
        assert_eq!((String::from("8"), String::from("wothree")), parse1("eightwothree"))
    }

    #[test]
    fn parse1_token_number_word_after_garbage() {
        assert_eq!((String::from("wo"), String::from("three")), parse1("wothree"));
    }

    #[test]
    fn parse1_empty_line() {
        let expected = (String::from(""), String::from(""));
        assert_eq!(expected, parse1(""));
    }

    #[test]
    fn parse_all_foo() {
        assert_eq!(
            vec![String::from("8"), String::from("wo"), String::from("3")],
            parse_all("eightwothree")
        );
    }
}
