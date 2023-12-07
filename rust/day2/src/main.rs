mod part1;
mod part2;

fn main() -> std::io::Result<()> {
    let lines: Vec<String> = std::io::stdin()
        .lines()
        .collect::<Result<_, _>>()?;
    if lines.is_empty() {
        eprintln!("Nothing found in stdin!");
        std::process::exit(1);
    }
    let mut games: Vec<part1::Game> = Vec::new();
    for l in lines {
        games.push(part1::parse_game(&l));
    }
    let valid_games_id_sum: usize = games
        .iter()
        .filter(|g| part1::is_game_possible(g))
        .map(|g| g.id)
        .sum();

    println!("{}", valid_games_id_sum);

    let sum = part2::sum_min_power_of_max_sets(&games[..]);
    println!("{}", sum);
    Ok(())
}
