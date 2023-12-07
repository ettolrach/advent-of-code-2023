
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Set {
    pub red: Option<usize>,
    pub green: Option<usize>,
    pub blue: Option<usize>,
}
impl Set {
    pub fn new() -> Set {
        Set {
            red: None,
            green: None,
            blue: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Game {
    pub id: usize,
    // "revelations" because these are the sets which are "revealed".
    pub revelations: Vec<Set>,
}

const CONFIGURATION: Set = Set { red: Some(12), green: Some(13), blue: Some(14) };

pub fn parse_game(s: &str) -> Game {
    let mut colon_split = s.split(':');
    let game_half = colon_split.next().unwrap();
    let revelations_half = colon_split.next().unwrap();

    let id: usize = (&game_half[5..(game_half.len())]).parse().unwrap();

    let mut revelations: Vec<Set> = Vec::new();
    for revelation in revelations_half.split(';') {
        let mut new_set = Set::new();
        for colour_text in revelation.split(',') {
            let mut colour_iter = colour_text.trim().split(' ');
            let num = colour_iter.next().unwrap();
            let colour = colour_iter.next().unwrap();
            match colour {
                "red" => new_set.red = Some(num.parse().unwrap()),
                "green" => new_set.green = Some(num.parse().unwrap()),
                "blue" => new_set.blue = Some(num.parse().unwrap()),
                _ => unreachable!(),
            }
        }
        revelations.push(new_set);
    }

    Game { id, revelations }
}

pub fn is_game_subseteq_of(a: &Set, b: &Set) -> bool {
    if a.red > b.red {
        return false;
    }
    if a.green > b.green {
        return false;
    }
    if a.blue > b.blue {
        return false;
    }
    true
}

pub fn is_game_possible(g: &Game) -> bool {
    for r in &g.revelations {
        if !is_game_subseteq_of(&r, &CONFIGURATION) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity_check() {
        assert_eq!(20, 12 + 8)
    }
    
    #[test]
    fn check_parse() {
        let control_game = Game {
            id: 1,
            revelations: vec![
                Set { red: Some(4), green: None, blue: Some(3) },
                Set { red: Some(1), green: Some(2), blue: Some(6) },
                Set { red: None, green: Some(2), blue: None },
            ],
        };
        let test_str = String::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        assert_eq!(parse_game(&test_str), control_game);
    }

    #[test]
    fn example() {
        let input = String::from(
"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
        );
        let mut games: Vec<Game> = Vec::new();
        for line in input.lines() {
            games.push(parse_game(line));
        }
        let id_sum: usize = games.iter().filter(|g| is_game_possible(g)).map(|g| g.id).sum();
        assert_eq!(8, id_sum);
    }
}
