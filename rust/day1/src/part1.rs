#[derive(Debug)]
pub struct NoNumbersPresent;

fn decode_calibration_value(s: &str) -> Result<u32, NoNumbersPresent> {
    let numbers: Vec<char> = s.chars()
        .filter(|c| c.is_ascii_digit())
        .collect();
    match (numbers.first(), numbers.last()) {
        (None, _) | (_, None) => Err(NoNumbersPresent),
        (Some(first), Some(last)) => {
            let temp_string = format!("{}{}", first, last);
            // Unwrapping here is safe because first and last were checked to be ASCII digits.
            Ok(temp_string.parse().unwrap())
        },
    }
}

pub fn correct_calibration_document(l: &[&str]) -> Result<Vec<u32>, NoNumbersPresent> {
    l.iter()
        .map(|s| Ok(decode_calibration_value(s)?))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::part1::correct_calibration_document;

    #[test]
    fn example() {
        let document = vec![
            "1abc2",
            "pqr3stu8vwx",
            "a1b2c3d4e5f",
            "treb7uchet",
        ];
        let decoded_document = correct_calibration_document(&document[..]).unwrap();
        let expected_nums: Vec<u32> = vec![12, 38, 15, 77];
        assert_eq!(decoded_document, expected_nums);

        let sum: u32 = decoded_document.iter().sum();
        let expected_sum: u32 = expected_nums.iter().sum();
        assert_eq!(sum, expected_sum);
    }
}
