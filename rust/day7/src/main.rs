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
        let lines_str_clone = lines_str.clone();
        let mut hands: Vec<part1::CamelHand> = lines_str_clone
            .iter()
            .map(|s| part1::string_to_camel_hand(s))
            .collect();
        part1::sort_camel_hands(&mut hands);
        let total_winnings = part1::calculate_total_winnings(&hands[..]);
        println!("{}", total_winnings)
    }

    {
        let lines_str_clone = lines_str.clone();
        let mut hands: Vec<part2::CamelHand> = lines_str_clone
            .iter()
            .map(|s| part2::string_to_camel_hand(s))
            .collect();
        part2::sort_camel_hands(&mut hands);
        let total_winnings = part2::calculate_total_winnings(&hands[..]);
        println!("{}", total_winnings)
    }
    Ok(())
}
