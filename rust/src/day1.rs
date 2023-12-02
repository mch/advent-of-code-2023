pub fn puzzle(input: &str) {
    let mut _lines = input.lines();

    let answer = 42;
    println!("The answer is {}", answer);
}

fn recover_calibration_value(line: &str) -> String {
    let matches: Vec<&str> = line.matches(char::is_numeric).collect();
    matches.join("")
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

}
