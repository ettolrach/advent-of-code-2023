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
    let lines_clone = lines.clone();
    let lines_str: Vec<&str> = lines_clone.iter().map(|s| s.as_str()).collect();
    {
        let (seeds, mapper) = part1::parse_input_lines(&lines_str[..]).unwrap();
        let lowest: usize = seeds.iter().map(|seed| mapper.map_all(*seed)).min().unwrap();
        
        println!("{}", lowest);
    }
    {
        let (_, mapper) = part1::parse_input_lines(&lines_str[..]).unwrap();
        let seeds = part2::parse_seeds_line(&lines[0]).unwrap();
        let lowest: usize = seeds.iter().map(|seed| mapper.map_all(*seed)).min().unwrap();
        println!("{}", lowest);
    }
    Ok(())
}
