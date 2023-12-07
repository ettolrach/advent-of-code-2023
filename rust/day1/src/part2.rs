use std::{str::FromStr, fmt::Display};

use crate::part1;

#[derive(Debug)]
enum DigitParseError {
    TooManyDigits,
    NotInt(std::num::ParseIntError)
}

struct Digit(u32);
impl FromStr for Digit {
    type Err = DigitParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 1 && s[0..1].parse::<u32>().is_ok() {
            return Ok(Digit(s[0..1].parse::<u32>().unwrap()));
        }
        match s.to_lowercase().as_str() {
            "one" => Ok(Digit(1)),
            "two" => Ok(Digit(2)),
            "three" => Ok(Digit(3)),
            "four" => Ok(Digit(4)),
            "five" => Ok(Digit(5)),
            "six" => Ok(Digit(6)),
            "seven" => Ok(Digit(7)),
            "eight" => Ok(Digit(8)),
            "nine" => Ok(Digit(9)),
            _ => match s.parse::<u32>() {
                Ok(_) => Err(DigitParseError::TooManyDigits),
                Err(e) => Err(DigitParseError::NotInt(e)),
            },
        }
    }
}
impl Display for Digit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Digit {
    fn as_num(&self) -> u32 {
        self.0
    }
}

const STRING_DIGITS: [&str; 18] = [
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "1",
    "2",
    "3",
    "4",
    "5",
    "6",
    "7",
    "8",
    "9",
];

type Index = usize;

fn decode_calibration_value(s: &str) -> Result<u32, part1::NoNumbersPresent> {
    // This will keep searching a string with the current query to find all occurrances.
    fn go(s: &str, to_find: &str) -> Vec<Index> {
        let mut find_result = s.find(to_find);
        let mut to_return: Vec<Index> = Vec::new();
        let mut offset: usize = 0;
        while find_result.is_some() {
            let latest_result = find_result.unwrap();
            to_return.push(latest_result + offset);
            if latest_result + offset + 1 >= s.len() {
                break;
            }
            else {
                find_result = (&s[(latest_result + 1 + offset)..(s.len())]).find(to_find);
                offset = latest_result + offset + 1;
            }
        }
        to_return
    }
    let mut findings: Vec<(Index, Digit)> = Vec::new();
    for d in STRING_DIGITS {
        findings.extend(go(s, d).iter().map(|i| (*i, Digit::from_str(d).unwrap())));
    }
    if findings.is_empty() {
        Err(part1::NoNumbersPresent)
    }
    else if findings.len() == 1 {
        Ok(format!("{}{}", findings[0].1.as_num(), findings[0].1.as_num()).parse::<u32>().unwrap())
    }
    else {
        let first = findings.iter().min_by_key(|tuple| tuple.0).unwrap();
        let last = findings.iter().max_by_key(|tuple| tuple.0).unwrap();
        Ok(format!("{}{}", first.1.to_string(), last.1.to_string()).parse::<u32>().unwrap())
    }
}

pub fn correct_calibration_document(l: &[&str]) -> Result<Vec<u32>, part1::NoNumbersPresent> {
    l.iter()
        .map(|s| Ok(decode_calibration_value(s)?))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::part2::correct_calibration_document;

    #[test]
    fn example() {
        let document = vec![
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ];
        let decoded_document = correct_calibration_document(&document[..]).unwrap();
        let expected_nums: Vec<u32> = vec![29, 83, 13, 24, 42, 14, 76];
        assert_eq!(decoded_document, expected_nums);

        let sum: u32 = decoded_document.iter().sum();
        let expected_sum: u32 = expected_nums.iter().sum();
        assert_eq!(sum, expected_sum);
    }
    
    #[test]
    fn tricky_one() {
        let document = vec!["5ffour295"];
        let decoded_document = correct_calibration_document(&document[..]).unwrap();
        let sum: u32 = decoded_document.iter().sum();
        assert_eq!(55, sum);
    }
}
