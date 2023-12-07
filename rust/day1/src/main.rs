use std::io;

use part1::NoNumbersPresent;

mod part1;
mod part2;

fn main() -> io::Result<()> {
    let lines: Vec<String> = io::stdin()
        .lines()
        .collect::<Result<_, _>>()?;
    if lines.is_empty() {
        eprintln!("Nothing found in stdin!");
        std::process::exit(1);
    }
    let lines_ref: Vec<&str> = lines.iter().map(|s| s.as_ref()).collect();

    // Part 1.
    {
        let corrected_document = match part1::correct_calibration_document(&lines_ref[..]) {
            Ok(l) => l,
            Err(NoNumbersPresent) => {
                eprintln!("No numbers were present in one of the lines of the document.");
                std::process::exit(1);
            },
        };
        println!("{}", corrected_document.into_iter().sum::<u32>());
    }

    // Part 2.
    {
        let corrected_document = match part2::correct_calibration_document(&lines_ref[..]) {
            Ok(l) => l,
            Err(NoNumbersPresent) => {
                eprintln!("No numbers were present in one of the lines of the document.");
                std::process::exit(1);
            },
        };
        println!("{}", corrected_document.into_iter().sum::<u32>());
    }
    Ok(())
}
