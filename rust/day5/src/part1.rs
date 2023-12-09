#[derive(Debug, Clone, Copy, PartialEq)]
struct Range {
    from: usize,
    to: usize,
    range: usize,
}
impl Range {
    fn convert(&self, n: usize) -> Option<usize> {
        if n >= self.from && n < self.from + self.range {
            Some(self.to + (n - self.from))
        }
        else {
            None
        }
    }
}

struct Map {
    ranges: Vec<Range>,
}
impl Map {
    fn map(&self, to_map: usize) -> usize {
        // This will loop through the ranges, try each one.
        // If it successfully mapped, then return.
        for r in &self.ranges {
            match r.convert(to_map) {
                Some(n) => return n,
                None => (),
            }
        }
        // If the map never worked, then simply use the same number.
        to_map
    }
}

pub struct Mapper {
    maps: Vec<Map>
}
impl Mapper {
    pub fn map_all(&self, to_map: usize) -> usize {
        self.maps.iter().fold(to_map, |num, map| map.map(num))
    }
}

pub type Seed = usize;

#[derive(Debug)]
pub struct InvalidInput;

pub fn parse_input_lines(s: &[&str]) -> Result<(Vec<Seed>, Mapper), InvalidInput> {
    if s.is_empty() {
        return Err(InvalidInput)
    }
    if !s[0].starts_with("seeds: ") {
        return Err(InvalidInput)
    }
    let mut seeds: Vec<Seed> = Vec::new();
    let seeds_str_iter = s[0][7..(s[0].len())].split(' ');
    for seed in seeds_str_iter {
        match seed.parse::<usize>() {
            Ok(n) => seeds.push(n),
            Err(_) => return Err(InvalidInput),
        }
    }

    let mut maps: Vec<Map> = Vec::new();

    let mut input_iter = s[2..(s.len())].iter();
    let mut next_line = input_iter.next();
    let mut next_map = Map {
        ranges: Vec::new(),
    };
    while next_line.is_some() {
        let line = next_line.unwrap();
        if line == &"" {
            maps.push(next_map);
            next_map = Map { ranges: Vec::new() }
        }
        else if !line.ends_with(":") {
            let mut ranges_iter = line.split(' ');
            let to = match ranges_iter.next().unwrap().parse::<usize>() {
                Ok(n) => n,
                Err(_) => return Err(InvalidInput),
            };
            let from = match ranges_iter.next().unwrap().parse::<usize>() {
                Ok(n) => n,
                Err(_) => return Err(InvalidInput),
            };
            let range = match ranges_iter.next().unwrap().parse::<usize>() {
                Ok(n) => n,
                Err(_) => return Err(InvalidInput),
            };
            next_map.ranges.push(Range { to, from, range });
        }
        next_line = input_iter.next();
    }
    maps.push(next_map);
    if maps.len() != 7 {
        return Err(InvalidInput)
    }

    let mapper = Mapper { maps };
    Ok((seeds, mapper))
}

#[cfg(test)]
mod tests {
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
        let lines: Vec<&str> = input.lines().collect();
        let (seeds, mapper) = parse_input_lines(&lines[..]).unwrap();
        let locations: Vec<usize> = seeds.iter().map(|seed| mapper.map_all(*seed)).collect();
        let actual_locations: Vec<usize> = vec![82, 43, 86, 35];
        assert_eq!(actual_locations, locations);

        let lowest: usize = *locations.iter().min().unwrap();
        let actual_lowest: usize = 35;
        assert_eq!(actual_lowest, lowest);
    }
}
