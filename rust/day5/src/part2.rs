use crate::part1::*;

pub fn parse_seeds_line(s: &str) -> Result<Vec<Seed>, InvalidInput> {
	let mut to_return: Vec<Seed> = Vec::new();
	let mut seeds_iter = s[7..(s.len())].split(' ');
	let mut next_seed_pair = (seeds_iter.next(), seeds_iter.next());
	while next_seed_pair.0.is_some() && next_seed_pair.1.is_some() {
		let start: usize = match next_seed_pair.0.unwrap().parse() {
			Ok(n) => n,
			Err(_) => return Err(InvalidInput),
		};
		let range: usize = match next_seed_pair.1.unwrap().parse() {
			Ok(n) => n,
			Err(_) => return Err(InvalidInput),
		};
		for i in start..(start + range) {
			to_return.push(i);
		}
		next_seed_pair = (seeds_iter.next(), seeds_iter.next());
		eprintln!("{:?}", next_seed_pair);
	}
	Ok(to_return)
}

#[cfg(test)]
mod tests {
    use crate::part1::*;
    use super::*;

    #[test]
    fn example() {
        let input = String::from(
"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
        );
		let input_clone = input.clone();
        let lines: Vec<&str> = input_clone.lines().collect();
        let (_, mapper) = parse_input_lines(&lines[..]).unwrap();

		let seeds_str = input.lines().next().unwrap();
		let seeds = parse_seeds_line(seeds_str).unwrap();



        let lowest: usize = seeds.iter().map(|seed| mapper.map_all(*seed)).min().unwrap();
        let actual_lowest: usize = 46;
        assert_eq!(actual_lowest, lowest);

	}
}