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
    println!("original line: {}", line);
    let massaged_line = parse_all(line);
    println!("massaged line: {}", massaged_line);
    let recovered = recover_calibration_value(&massaged_line);
    println!("recovered: {}", recovered);
    recovered
}

fn recover_calibration_value(line: &str) -> String {
    let matches: Vec<&str> = line.matches(char::is_numeric).collect();
    format!("{}{}", matches[0], matches[matches.len() - 1])
}

fn parse1(line: &str) -> (String, String) {
    let mut number_words = HashMap::new();
    number_words.insert(String::from("one"), String::from("1"));
    number_words.insert(String::from("two"), String::from("2"));
    number_words.insert(String::from("three"), String::from("3"));
    number_words.insert(String::from("four"), String::from("4"));
    number_words.insert(String::from("five"), String::from("5"));
    number_words.insert(String::from("six"), String::from("6"));
    number_words.insert(String::from("seven"), String::from("7"));
    number_words.insert(String::from("eight"), String::from("8"));
    number_words.insert(String::from("nine"), String::from("9"));

    let mut min = line.len();
    for number_word in number_words.keys() {
        if let Some(offset) = line.find(number_word) {
            if offset == 0 {
                let token = number_word;
                let token_value = number_words.get(token).unwrap();
                let token_length = token.len();
                return (token_value.clone(), String::from(&line[token_length..line.len()]))
            } else if offset < min {
                min = offset;
            }
        }
    }
    if min > 0 {
        let gibberish = String::from(&line[0..min]);
        let rest = String::from(&line[min..line.len()]);
        return (gibberish, rest);
    }
    (String::from(line), String::from(""))
}

fn parse_all(line: &str) -> String {
    let mut result: Vec<String> = vec![];
    let mut rest = String::from(line);
    while rest.len() > 0 {
        let (token, rest2) = parse1(&rest);
        result.push(token);
        rest = rest2;
    }

    result.join("")
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
    fn summed_calibration_values_part2() {
        let input = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        assert_eq!("281", puzzle(input));
    }

    #[test]
    fn test_replace_word_with_number_1() {
        let line = "two1nine";
        let expected = "219";

        let result = parse_all(line);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_replace_word_with_number_2() {
        let line = "eightwothree";
        let expected = "8wo3";

        let result = parse_all(line);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_replace_word_with_number_and_recover_calibration_value() {
        let line = "two1nine";
        let expected = "29";

        let result = parse_all(line);
        assert_eq!(expected, recover_calibration_value(&result));
    }

    #[test]
   fn test_recover_massaged_calibration_values1() {
        assert_eq!("29", recover_massaged_calibration_value("two1nine"));
   }

    #[test]
    fn test_recover_massaged_calibration_values2() {
        assert_eq!("83", recover_massaged_calibration_value("eightwothree"));
    }

    #[test]
    fn test_recover_massaged_calibration_values3() {
        assert_eq!("13", recover_massaged_calibration_value("abcone2threexyz"));
    }

    #[test]
    fn test_recover_massaged_calibration_values4() {
        assert_eq!("24", recover_massaged_calibration_value("xtwone3four"));
    }

    #[test]
    fn test_recover_massaged_calibration_values5() {
        assert_eq!("42", recover_massaged_calibration_value("4nineeightseven2"));
    }

    #[test]
    fn test_recover_massaged_calibration_values6() {
        assert_eq!("14", recover_massaged_calibration_value("zoneight234"));
    }

    #[test]
    fn test_recover_massaged_calibration_values7() {
        assert_eq!("76", recover_massaged_calibration_value("7pqrstsixteen"));
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
    fn parse1_gibberish() {
        let expected = (String::from("xyz"), String::from(""));
        assert_eq!(expected, parse1("xyz"));
    }

    #[test]
    fn parse1_wtf() {
        let expected = (String::from("x"), String::from("twone3four"));
        assert_eq!(expected, parse1("xtwone3four"));
    }

    #[test]
    fn parse1_wtf2() {
        let expected = (String::from("2"), String::from("ne3four"));
        assert_eq!(expected, parse1("twone3four"));
    }

    #[test]
    fn parse_all_foo1() {
        assert_eq!(String::from("219"), parse_all("two1nine"));
    }

    #[test]
    fn parse_all_foo2() {
        assert_eq!(String::from("8wo3"), parse_all("eightwothree"));
    }

    #[test]
    fn parse_all_foo3() {
        assert_eq!(String::from("abc123xyz"), parse_all("abcone2threexyz"));
    }

    #[test]
    fn parse_all_foo4() {
        assert_eq!(String::from("x2ne34"), parse_all("xtwone3four"));
    }

    #[test]
    fn parse_all_foo5() {
        assert_eq!(String::from("49872"), parse_all("4nineeightseven2"));
    }

    #[test]
    fn parse_all_foo6() {
        assert_eq!(String::from("z1ight234"), parse_all("zoneight234"));
    }

    #[test]
    fn parse_all_foo7() {
        assert_eq!(String::from("7pqrst6teen"), parse_all("7pqrstsixteen"));
    }
}
