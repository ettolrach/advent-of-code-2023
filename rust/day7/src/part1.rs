use std::str::FromStr;

#[derive(Debug)]
pub struct CardParseError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}
impl FromStr for Card {
    type Err = CardParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "2" => Ok(Card::Two),
            "3" => Ok(Card::Three),
            "4" => Ok(Card::Four),
            "5" => Ok(Card::Five),
            "6" => Ok(Card::Six),
            "7" => Ok(Card::Seven),
            "8" => Ok(Card::Eight),
            "9" => Ok(Card::Nine),
            "T" => Ok(Card::Ten),
            "J" => Ok(Card::Jack),
            "Q" => Ok(Card::Queen),
            "K" => Ok(Card::King),
            "A" => Ok(Card::Ace),
            _ => Err(CardParseError),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Category {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

pub fn all_equal<A>(slice: &[A]) -> bool
where
    A: PartialEq
{
    if slice.is_empty() {
        return false
    }
    slice.iter().all(|a| a == &slice[0])
}

pub struct VecMap<K, V> {
    pub vec: Vec<(K, V)>
}
impl<K, V> VecMap<K, V> {
    pub fn new() -> VecMap<K, V>
    where
        K: PartialEq,
    {
        VecMap { vec: Vec::new() }
    }
    pub fn from_tuples_slice(tuples: &[(K, V)]) -> VecMap<K, V>
    where
        K: Clone,
        K: PartialEq,
        V: Clone,
    {
        VecMap { vec: Vec::from(tuples) }
    }
    pub fn get_value(&self, key: &K) -> Option<&V>
    where
        K: PartialEq,
    {
        match self.get_id_from_key(key) {
            Some(i) => Some(&self.vec[i].1),
            None => None,
        }
    }
    fn get_id_from_key(&self, key: &K) -> Option<usize>
    where
        K: PartialEq,
    {
        for i in 0..(self.vec.len()) {
            if &self.vec[i].0 == key {
                return Some(i)
            }
        }
        None
    }
    pub fn update(&mut self, tuple: (K, V))
    where
        K: PartialEq
    {
        match self.get_id_from_key(&tuple.0) {
            Some(i) => self.vec[i].1 = tuple.1,
            None => self.vec.push(tuple),
        }
    }
    pub fn update_with_fn(&mut self, key: K, identity: V, func: impl FnOnce(&V) -> V)
    where
        K: PartialEq
    {
        match self.get_id_from_key(&key) {
            Some(i) => self.vec[i].1 = func(&self.vec[i].1),
            None => self.update((key, identity)),
        }
    }
    pub fn keys(&self) -> Vec<&K> {
        self.vec.iter().map(|tuple| &tuple.0).collect()
    }
    pub fn values(&self) -> Vec<&V> {
        self.vec.iter().map(|tuple| &tuple.1).collect()
    }
}

pub fn get_category(hand: &[Card; 5]) -> Category {
    let mut vec_map: VecMap<Card, usize> = VecMap::new();
    for card in hand {
        vec_map.update_with_fn(*card, 1, |n| n + 1);
    }
    let count: Vec<usize> = vec_map.values().iter().map(|n| **n).collect();
    if count.contains(&5) {
        return Category::FiveOfAKind
    }
    if count.contains(&4) {
        return Category::FourOfAKind
    }
    if count.contains(&3) && count.contains(&2) {
        return Category::FullHouse
    }
    if count.contains(&3) {
        return Category::ThreeOfAKind
    }
    if count.iter().filter(|n| n == &&2).count() == 2 {
        return Category::TwoPair
    }
    if count.contains(&2) {
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
        let lowest = Hand::from_str("12344");
        let middle = Hand::from_str("1KQQQ");
        let high = Hand::from_str("21QQQ");
        let highest = Hand::from_str("555AA");
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
        let expected_total_winnings: usize = 6440;
        assert_eq!(expected_total_winnings, total_winnings);
    }
}
