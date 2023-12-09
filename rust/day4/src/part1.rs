use std::str::FromStr;

#[derive(Debug)]
pub struct InvalidFormat;

pub struct Card {
    id: usize,
    winners: Vec<usize>,
    guesses: Vec<usize>,
}
impl Card {
    fn get_winning_numbers(&self) -> Vec<usize> {
        self.guesses
            .iter()
            .filter(|n| self.winners.contains(n))
            .map(|n| *n)
            .collect()
    }
    pub fn get_score(&self) -> usize {
        match self.get_winning_numbers().len() {
            0 => 0,
            // Argument to pow() must be of type u32 for some reason,
            // so convert to u32 and then back.
            length => 2_u32.pow((length as u32) - 1) as usize,
        }
    }
}
impl FromStr for Card {
    type Err = InvalidFormat;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut colon_split = s.split(':');
        let card_part = match colon_split.next() {
            Some(str) => str,
            None => return Err(InvalidFormat),
        };
        let other_part = match colon_split.next() {
            Some(str) => str,
            None => return Err(InvalidFormat),
        };
        let id: usize = (&card_part[5..(card_part.len())]).trim().parse().unwrap();
        let mut pipe_split = other_part.split('|');
        let winning_numbers_str = match pipe_split.next() {
            Some(str) => str,
            None => return Err(InvalidFormat),
        };
        let guessed_numbers_str = match pipe_split.next() {
            Some(str) => str,
            None => return Err(InvalidFormat),
        };
        let winning_numbers: Vec<usize> = winning_numbers_str
            .split(' ')
            .filter(|s| s != &"")
            .map(|s| s.parse().unwrap())
            .collect();
        let guessed_numbers: Vec<usize> = guessed_numbers_str
            .split(' ')
            .filter(|s| s != &"")
            .map(|s| s.parse().unwrap())
            .collect();

        Ok(Card {
            id,
            winners: winning_numbers,
            guesses: guessed_numbers,
        })

    }
}

pub fn cards_from_lines(slice: &[&str]) -> Vec<Card> {
    slice.iter()
        .map(|s| Card::from_str(s).unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = String::from(
"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
        );
        let lines: Vec<&str> = input.lines().collect();
        let cards: Vec<Card> = cards_from_lines(&lines[..]);
        let scores: Vec<usize> = cards.iter().map(|card| card.get_score()).collect();
        let expected_scores: Vec<usize> = vec![8, 2, 2, 1, 0, 0];
        assert_eq!(expected_scores, scores);
        let sum: usize = cards.iter()
            .map(|card| card.get_score())
            .sum();
        let expected_sum: usize = 13;
        assert_eq!(expected_sum, sum);
    }
}