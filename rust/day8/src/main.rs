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
    println!("{}", part1::calculate_how_many_steps(&lines_str[..]));
    println!("{}", part2::calculate_how_many_steps(&lines_str[..]));
    Ok(())
}
