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
impl ToString for Card {
    fn to_string(&self) -> String {
        String::from(match self {
            Card::Joker => "J",
            Card::Two => "2",
            Card::Three => "3",
            Card::Four => "4",
            Card::Five => "5",
            Card::Six => "6",
            Card::Seven => "7",
            Card::Eight => "8",
            Card::Nine => "9",
            Card::Ten => "T",
            Card::Queen => "Q",
            Card::King => "K",
            Card::Ace => "A",
        })
    }
}

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
    // println!("vec_map:");
    // for card_count in vec_map.vec.clone() {
    //     println!("{:?}", card_count);
    // }
    let count: Vec<usize> = vec_map.values().iter().map(|n| **n).collect();
    let joker_count: usize  = match vec_map.get_value(&Card::Joker) {
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
        || ((count.iter().filter(|n| n == &&2).count() == 2) && joker_count == 2)
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
        || (count.contains(&2) && joker_count == 1 && (count.iter().filter(|n| n == &&1).count() == 3))
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
    // println!("{:?}", (0..(hands.len())));
    // println!("mapped:");
    // for i in (0..(hands.len())).map(|i| hands[i].1 * (i + 1)) {
    //     println!("{}", i);
    // }
    // let sum = (0..(hands.len())).map(|i| hands[i].1 * (i + 1)).sum::<usize>();
    // let mut running_total: usize = 0;
    // for n in (0..(hands.len())).map(|i| hands[i].1 * (i + 1)) {
    //     running_total += n;
    //     println!("eee{}", n);
    // }
    // println!("running_total: {:?}", running_total);
    // println!("sum: {:?}", sum);
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

    #[test]
    fn category_116() {
        let hand: CamelHand = (Hand::from_str("Q9TQJ"), 116);
        let expected_category = Category::ThreeOfAKind;
        assert_eq!(expected_category, hand.0.category);
    }
    #[test]
    fn category_152() {
        let hand: CamelHand = (Hand::from_str("Q4J94"), 152);
        let expected_category = Category::ThreeOfAKind;
        assert_eq!(expected_category, hand.0.category);
    }
    #[test]
    fn category_193() {
        let hand: CamelHand = (Hand::from_str("55T8T"), 193);
        let expected_category = Category::TwoPair;
        assert_eq!(expected_category, hand.0.category);
    }

    #[test]
    fn input_extract() {
        let input = String::from(
"JJJJ8 619
Q4J94 152
77587 277
7333J 651
QQQQ2 419
72KA3 851
555Q2 806
37QTT 72
39446 597
KK99T 453
T5522 247
8TK48 109
46J82 146
444A7 788
Q9TQJ 116
3A9AA 529
5AAAJ 63
T9522 668
ATJTJ 879
7TATT 11"
        );
        let mut hands: Vec<CamelHand> = input.lines().map(|s| string_to_camel_hand(s)).collect();
        sort_camel_hands(&mut hands);
        let total_winnings = calculate_total_winnings(&hands[..]);
        let sorted: Vec<CamelHand> = String::from(
"72KA3 851
37QTT 72
39446 597
46J82 146
8TK48 109
T9522 668
T5522 247
KK99T 453
3A9AA 529
444A7 788
555Q2 806
77587 277
7TATT 11
Q4J94 152
Q9TQJ 116
5AAAJ 63
7333J 651
QQQQ2 419
ATJTJ 879
JJJJ8 619"
        ).lines().map(|s| string_to_camel_hand(s)).collect();
        let expected_total_winnings: usize = 90816;
        assert_eq!(sorted.iter().map(|tuple| tuple.0.category).collect::<Vec<_>>(), hands.iter().map(|tuple| tuple.0.category).collect::<Vec<_>>());
        assert_eq!(expected_total_winnings, total_winnings);
    }

    #[test]
    fn bigger_input_extract() {
        let input = String::from(
"JJJJ8 619
Q4J94 152
77587 277
7333J 651
QQQQ2 419
72KA3 851
555Q2 806
37QTT 72
39446 597
KK99T 453
T5522 247
8TK48 109
46J82 146
444A7 788
Q9TQJ 116
3A9AA 529
5AAAJ 63
T9522 668
ATJTJ 879
7TATT 11
88686 885
5QJ55 782
72K77 576
KQ48J 352
JJ488 704
3K356 12
JQJAQ 201
26272 373
88JJ2 855
35333 167
755Q4 465
5J6T5 136
JTA23 477
J8488 252
55556 417
55T8T 193
22782 148
2372J 811
J4K72 114
9Q4KK 303"
        );
        let mut hands: Vec<CamelHand> = input.lines().map(|s| string_to_camel_hand(s)).collect();
        sort_camel_hands(&mut hands);
        let total_winnings = calculate_total_winnings(&hands[..]);
        let sorted: Vec<CamelHand> = String::from(
"72KA3 851
J4K72 114
JTA23 477
37QTT 72
39446 597
3K356 12
46J82 146
755Q4 465
8TK48 109
9Q4KK 303
T9522 668
KQ48J 352
55T8T 193
T5522 247
KK99T 453
22782 148
2372J 811
26272 373
3A9AA 529
444A7 788
5J6T5 136
555Q2 806
72K77 576
77587 277
7TATT 11
Q4J94 152
Q9TQJ 116
88686 885
JJ488 704
J8488 252
JQJAQ 201
35333 167
55556 417
5QJ55 782
5AAAJ 63
7333J 651
88JJ2 855
QQQQ2 419
ATJTJ 879
JJJJ8 619"
        ).lines().map(|s| string_to_camel_hand(s)).collect();
        let expected_totals: Vec<usize> = vec![851, (114 * 2), (477 * 3), (72 * 4), (597 * 5), (12 * 6), (146 * 7), (465 * 8), (109 * 9), (303 * 10), (668 * 11), (352 * 12), (193 * 13), (247 * 14), (453 * 15), (148 * 16), (811 * 17), (373 * 18), (529 * 19), (788 * 20), (136 * 21), (806 * 22), (576 * 23), (277 * 24), (11 * 25), (152 * 26), (116 * 27), (885 * 28), (704 * 29), (252 * 30), (201 * 31), (167 * 32), (417 * 33), (782 * 34), (63 * 35), (651 * 36), (855 * 37), (419 * 38), (879 * 39), (619 * 40)];
        let totals: Vec<usize> = (0..(hands.len())).map(|i| hands[i].1 * (i + 1)).collect();
        assert_eq!(expected_totals, totals);
        let expected_total_winnings: usize = 372384;
        assert_eq!(sorted.iter().map(|tuple| tuple.0.category).collect::<Vec<_>>(), hands.iter().map(|tuple| tuple.0.category).collect::<Vec<_>>());
        assert_eq!(expected_total_winnings, total_winnings);
    }

    #[test]
    fn edge_cases() {
        let input = String::from(
"7788J 1
JJJJJ 2
JJ2JJ 3
87654 4
54995 5
9J33Q 6
KJ639 7"
        );
        let sorted: Vec<CamelHand> = String::from(
"87654 4
KJ639 7
54995 5
9J33Q 6
7788J 1
JJJJJ 2
JJ2JJ 3"
        ).lines().map(|s| string_to_camel_hand(s)).collect();
        let mut hands: Vec<CamelHand> = input.lines().map(|s| string_to_camel_hand(s)).collect();
        sort_camel_hands(&mut hands);
        let total_winnings = calculate_total_winnings(&hands[..]);
        let expected_total_winnings: usize = 95;
        assert_eq!(sorted, hands);
        assert_eq!(expected_total_winnings, total_winnings);

    }

    #[test]
    fn another_edge_case() {
        let input = String::from("J68J4 42");
        let (hand, _) = string_to_camel_hand(&input[..]);
        assert_eq!(Category::ThreeOfAKind, hand.category)
    }
}
