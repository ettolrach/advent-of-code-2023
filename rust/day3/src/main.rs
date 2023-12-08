mod part1;

fn main() -> std::io::Result<()> {
    let lines: Vec<String> = std::io::stdin()
        .lines()
        .collect::<Result<_, _>>()?;
    if lines.is_empty() {
        eprintln!("Nothing found in stdin!");
        std::process::exit(1);
    }
    let mut input = String::new();
    for l in lines {
        input.push_str(&l);
    }
    let part_numbers = part1::find_part_numbers(&input);
    let sum = part_numbers.iter().sum::<usize>();
    println!("{}", sum);
    Ok(())
}
