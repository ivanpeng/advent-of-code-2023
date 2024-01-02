use std::str::FromStr;
use regex::Regex;

pub fn get_digit_from_word(word: &str) -> Option<char> {
    if word.len() == 1 { return word.chars().nth(0) }
    return match word {
        "one" => Some('1'),
        "two" => Some('2'),
        "three" => Some('3'),
        "four" => Some('4'),
        "five" => Some('5'),
        "six" => Some('6'),
        "seven" => Some('7'),
        "eight" => Some('8'),
        "nine" => Some('9'),
        _ => None
    }

}

pub fn extract_digits(input: &str, regex: &Regex) -> Vec<char> {
    let mut a: Vec<char> = Vec::new();
    let it = regex.captures_iter(input).into_iter();
    for i in it {
        let digit = get_digit_from_word(i.get(0).unwrap().as_str());
        a.push(digit.unwrap())
    }

    return a
}


pub fn get_calibration_digits(digits: Vec<char>) -> Option<u32> {
    // get first and last digit
    if digits.len() == 0 { return None}
    let res = u32::from_str(&*(digits[0].to_string() + &*digits[digits.len()-1].to_string()));
    return Some(res.unwrap())
}

pub fn dec01_1(input: &str) -> u32 {
    let mut sum: u32 = 0;
    let regex = Regex::new(r"([1-9])").unwrap();
    for line in input.lines() {
        let val = get_calibration_digits(extract_digits(line, &regex));
        if val.is_some() {sum += val.unwrap();}

    }
    return sum
}

pub fn dec01_2(input: &str) -> u32 {
    let mut sum: u32 = 0;
    let regex = Regex::new(r"([1-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    for line in input.lines() {
        let val = get_calibration_digits(extract_digits(line, &regex));
        if val.is_some() {sum += val.unwrap()}

    }
    return sum
}

#[cfg(test)]
mod tests {
    use regex::Regex;
    use crate::dec01::{get_calibration_digits, extract_digits, dec01_1};

    #[test]
    fn test_parse_string_for_digits() {
        let regex = Regex::new(r"([1-9])").unwrap();

        assert_eq!(extract_digits("f12abc4", &regex), ['1', '2', '4']);
        assert_eq!(extract_digits("twone1", &regex), ['2', '1']);
    }

    #[test]
    fn test_parse_digits_returns_correctly() {
        let regex = Regex::new(r"([1-9])").unwrap();

        assert_eq!(get_calibration_digits(extract_digits("f12abc4", &regex)), Some(14));
        assert_eq!(get_calibration_digits(extract_digits("1abc2", &regex)), Some(12));
        assert_eq!(get_calibration_digits(extract_digits("pqr3stu8vwx", &regex)), Some(38));
        assert_eq!(get_calibration_digits(extract_digits("a1b2c3d4e5f", &regex)), Some(15));
        assert_eq!(get_calibration_digits(extract_digits("treb7uchet", &regex)), Some(77));
    }


    #[test]
    fn test_parse_input_returns_correctly() {
        assert_eq!(dec01_1("1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"), 142);
    }

    #[test]
    fn test_parse_pt_2_returns_correctly() {
        let s = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(dec01_1(s), 281)
    }
}