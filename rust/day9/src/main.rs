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
        let sum: isize = lines_str
            .iter()
            .map(|s| part1::str_to_sequence(s))
            .map(|nums| part1::next_in_sequence(&nums[..]))
            .sum();
        println!("{}", sum);
    }
    {
        let sum: isize = lines_str
            .iter()
            .map(|s| part2::str_to_sequence(s))
            .map(|nums| part2::previous_in_sequence(&nums[..]))
            .sum();
        println!("{}", sum);
    }
    Ok(())
}
