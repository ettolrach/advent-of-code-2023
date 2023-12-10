

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Race {
    pub time_allowed: usize,
    pub record_distance: usize,
}
impl Race {
    fn hold_to_distance(&self, held_time: usize) -> usize {
        // Remaining time * speed (where speed = held_time).
        (self.time_allowed - held_time) * held_time
    }
    pub fn number_of_ways_to_win(&self) -> usize {
        (1..(self.time_allowed))
            .map(|i| self.hold_to_distance(i))
            .filter(|distance| distance > &self.record_distance)
            .count()
    }
}

pub fn parse_numbers_string(s: &str) -> impl Iterator<Item = usize> + '_ {
    s
        .trim()
        .split(' ')
        .filter(|s| s != &"")
        .map(|s| s.parse::<usize>().unwrap())
}

pub fn parse_lines(slice: &[&str]) -> Vec<Race> {
    let times = parse_numbers_string(&slice[0][5..(slice[0].len())]);
    let distances = parse_numbers_string(&slice[1][9..(slice[1].len())]);
    
    let to_return: Vec<Race> = std::iter::zip(times, distances)
        .map(|tuple| Race { time_allowed: tuple.0, record_distance: tuple.1 })
        .collect();

    to_return
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
        let races = parse_lines(&(input.lines().collect::<Vec<&str>>())[..]);
        let product: usize = races.iter().map(|race| race.number_of_ways_to_win()).product();
        let expected_product: usize = 288;
        assert_eq!(expected_product, product);
    }
}
