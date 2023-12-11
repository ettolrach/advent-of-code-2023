use std::str::FromStr;
use crate::part1::{ Category, VecMap };

#[derive(Debug)]
pub struct CardParseError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Card {
	Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}
impl FromStr for Card {
    type Err = CardParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
			"J" => Ok(Card::Joker),
            "2" => Ok(Card::Two),
            "3" => Ok(Card::Three),
            "4" => Ok(Card::Four),
            "5" => Ok(Card::Five),
            "6" => Ok(Card::Six),
            "7" => Ok(Card::Seven),
            "8" => Ok(Card::Eight),
            "9" => Ok(Card::Nine),
            "T" => Ok(Card::Ten),
            "Q" => Ok(Card::Queen),
            "K" => Ok(Card::King),
            "A" => Ok(Card::Ace),
            _ => Err(CardParseError),
        }
    }
}

const NORMAL_CARDS: [Card; 12] = [
    Card::Two,
    Card::Three,
    Card::Four,
    Card::Five,
    Card::Six,
    Card::Seven,
    Card::Eight,
    Card::Nine,
    Card::Ten,
    Card::Queen,
    Card::King,
    Card::Ace,
];

pub fn get_category(hand: &[Card; 5]) -> Category {
    // let mut joker_positions: Vec<usize> = Vec::new();
    // for i in 0..5 {
    //     if hand[i] == Card::Joker {
    //         joker_positions.push(i);
    //     }
    // }
    // if joker_positions.len() == 5 {
    //     return Category::FiveOfAKind
    // }

    // let mut hands_to_check: Vec<[Card; 5]> = Vec::new();
    // for id in joker_positions {
    //     for c in NORMAL_CARDS {
    //         let mut new_hand = hand.clone();
    //         new_hand[id] = c;
    //         hands_to_check.push(new_hand)
    //     }
    // }

    let mut vec_map: VecMap<Card, usize> = VecMap::new();
    for card in hand {
        vec_map.update_with_fn(*card, 1, |n| n + 1);
    }
    let count: Vec<usize> = vec_map.values().iter().map(|n| **n).collect();
    let joker_count:usize  = match vec_map.get_value(&Card::Joker) {
        Some(n) => *n,
        None => 0,
    };
    let five_of_a_kind_rule: bool = count.contains(&5)
        || (count.contains(&4) && joker_count == 1)
        || (count.contains(&3) && joker_count == 2)
        || (count.contains(&2) && joker_count == 3)
        || (count.contains(&1) && joker_count == 4);
    if five_of_a_kind_rule {
        return Category::FiveOfAKind
    }
    let four_of_a_kind_rule: bool = count.contains(&4)
        || (count.contains(&3) && joker_count == 1)
        || (count.contains(&2) && joker_count == 2)
        || (count.contains(&1) && joker_count == 3);
    if four_of_a_kind_rule  {
        return Category::FourOfAKind
    }
    let full_house_rule: bool = (count.contains(&3) && count.contains(&2))
        || ((count.iter().filter(|n| n == &&2).count() == 2) && (joker_count == 1));
    if full_house_rule {
        return Category::FullHouse
    }
    let three_of_a_kind_rule: bool = count.contains(&3)
        || (count.contains(&2) && joker_count == 1 && (count.iter().filter(|n| n == &&1).count() == 2))
        || (count.contains(&1) && joker_count == 2);
    if three_of_a_kind_rule {
        return Category::ThreeOfAKind
    }
    let two_pair_rule: bool = count.iter().filter(|n| n == &&2).count() == 2;
    if two_pair_rule {
        return Category::TwoPair
    }
    let one_pair_rule: bool = count.contains(&2)
        || (count.iter().filter(|n| n == &&1).count() == 5) && joker_count == 1;
    if one_pair_rule {
        return Category::OnePair
    }
    Category::HighCard
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hand {
    cards: [Card; 5],
    category: Category
}
impl Hand {
    fn from_str(s: &str) -> Hand {
        assert!(s.len() == 5);
        let cards = s.chars()
            .map(|c| Card::from_str(&c.to_string()).unwrap())
            .collect::<Vec<Card>>()
            .try_into()
            .unwrap();
        Hand {
            cards,
            category: get_category(&cards),
        }
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.category.cmp(&other.category) {
            std::cmp::Ordering::Equal => {
                for i in 0..5 {
                    match self.cards[i].cmp(&other.cards[i]) {
                        std::cmp::Ordering::Equal => (),
                        ordering => return ordering,
                    }
                }
                std::cmp::Ordering::Equal
            },
            ordering => ordering,
        }
    }
}

pub type Bid = usize;
pub type CamelHand = (Hand, Bid);

pub fn sort_camel_hands(hands: &mut [CamelHand]) {
    hands.sort_by_key(|h| h.0)
}

pub fn calculate_total_winnings(hands: &[CamelHand]) -> usize {
    (0..(hands.len())).map(|i| hands[i].1 * (i + 1)).sum()
}

pub fn string_to_camel_hand(s: &str) -> CamelHand {
    let mut s_iter = s.split(' ');
    let hand = Hand::from_str(s_iter.next().unwrap());
    let bid: usize = s_iter.next().unwrap().parse().unwrap();
    (hand, bid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hand_ordering() {
        use Card::*;
        let lowest = Hand::from_str("2534J");
        let middle = Hand::from_str("2KQQJ");
        let high = Hand::from_str("32QQJ");
        let highest = Hand::from_str("55JAA");
        let control_cards = vec![lowest.clone(), middle.clone(), high.clone(), highest.clone()];
        let mut sorted_cards = vec![middle, high, highest, lowest];
        sorted_cards.sort();
        assert_eq!(control_cards, sorted_cards)
    }

    #[test]
    fn example() {
        let input = String::from(
"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
        );
        let mut hands: Vec<CamelHand> = input.lines().map(|s| string_to_camel_hand(s)).collect();
        sort_camel_hands(&mut hands);
        let total_winnings = calculate_total_winnings(&hands[..]);
        let expected_total_winnings: usize = 5905;
        assert_eq!(expected_total_winnings, total_winnings);
    }
}
