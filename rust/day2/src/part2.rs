use crate::part1::{ Game, Set };

pub fn order_of_set(s: &Set) -> Option<usize> {
    Some(s.red? * s.green? * s.blue?)
}

pub fn max_set_colours(sets: &[Set]) -> Set {
    let max_red = sets
        .iter()
        .filter(|s| s.red.is_some())
        .map(|s| s.red.unwrap())
        .max();

    let max_green = sets
        .iter()
        .filter(|s| s.green.is_some())
        .map(|s| s.green.unwrap())
        .max();

    let max_blue = sets
        .iter()
        .filter(|s| s.blue.is_some())
        .map(|s| s.blue.unwrap())
        .max();

    Set { red: max_red, green: max_green, blue: max_blue }
}

pub fn min_required_cubes(g: &Game) -> Game {
    let max_set = max_set_colours(&g.revelations[..]);
    Game { id: g.id, revelations: vec![max_set] }
}

pub fn min_powers(games: &[Game]) -> Vec<usize> {
    let mut to_return: Vec<usize> = Vec::new();
    for g in games {
        to_return.push(order_of_set(&max_set_colours(&g.revelations[..])).unwrap());
    }
    to_return
}

pub fn sum_min_power_of_max_sets(games: &[Game]) -> usize {
    games
        .iter()
        .map(|g|
            order_of_set(
                &max_set_colours(&g.revelations[..])
            ).unwrap()
        )
        .sum()
}


#[cfg(test)]
mod tests {
    use super::*;

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
            games.push(crate::part1::parse_game(line));
        }
        let sum = sum_min_power_of_max_sets(&games[..]);
        assert_eq!(2286, sum);
    }
}
