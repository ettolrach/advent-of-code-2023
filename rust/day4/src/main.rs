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
    let lines_str: Vec<&str> = lines.iter().map(|s| s.as_str()).collect();
    {
        let cards = part1::cards_from_lines(&lines_str[..]);
        let sum: usize = cards.iter()
            .map(|card| card.get_score())
            .sum();
        println!("{}", sum);
    }
    {
        let mut cards = part2::cards_from_lines(&lines_str[..]);
        part2::update_cards_after_winnings(&mut cards[..]);
        let sum: usize = part2::get_total_cards(&cards[..]);
        println!("{}", sum);
    }
    
    Ok(())
}
