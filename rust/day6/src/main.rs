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
        let races = part1::parse_lines(&lines_str[..]);
        let product: usize = races.iter().map(|race| race.number_of_ways_to_win()).product();
        println!("{}", product);
    }

    {
        let race = part2::parse_lines(&lines_str[..]);
        let number_of_ways: usize = race.number_of_ways_to_win();
        println!("{}", number_of_ways);
    }
    Ok(())
}
