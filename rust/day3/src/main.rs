mod part1;

fn main() -> std::io::Result<()> {
    let lines: Vec<String> = std::io::stdin()
        .lines()
        .collect::<Result<_, _>>()?;
    if lines.is_empty() {
        eprintln!("Nothing found in stdin!");
        std::process::exit(1);
    }
    let lines_str: Vec<&str> = lines.iter().map(|s| s.as_str()).collect();
    let mut grid = part1::Grid::from_lines(&lines_str[..]);
    let part_numbers: Vec<usize> = grid.get_part_numbers();
    let sum = part_numbers.iter().sum::<usize>();
    println!("{}", sum);
    Ok(())
}
