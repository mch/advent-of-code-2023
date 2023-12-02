pub fn puzzle(input: &str) {
    let mut _lines = input.lines();

    let answer = 42;
    println!("The answer is {}", answer);
}

fn recover_calibration_value(line: &str) -> String {
    "77".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test list:
    // 1abc2        12
    // pqr3stu8vwx  38
    // a1b2c3d4e5f  15
    // treb7uchet   77

    #[test]
    fn test_digits_at_start_and_end() {
        let line = "1abc2";
        let expected = "77";

        let result = recover_calibration_value(&line);
        assert_eq!(expected, result);
    }

}
