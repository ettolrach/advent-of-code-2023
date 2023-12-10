use crate::part1::Race;

pub fn parse_numbers_string(s: &str) -> usize {
    s
        .trim()
        .split(' ')
        .filter(|s| s != &"")
        .collect::<String>()
        .parse()
        .unwrap()
}

pub fn parse_lines(slice: &[&str]) -> Race {
    let time = parse_numbers_string(&slice[0][5..(slice[0].len())]);
    let distance = parse_numbers_string(&slice[1][9..(slice[1].len())]);
    
    Race { time_allowed: time, record_distance: distance }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = String::from(
"Time:      7  15   30
Distance:  9  40  200"
        );
        let race = parse_lines(&(input.lines().collect::<Vec<&str>>())[..]);
        let expected_number_of_ways: usize = 71503;
        assert_eq!(expected_number_of_ways, race.number_of_ways_to_win());
    }
}
