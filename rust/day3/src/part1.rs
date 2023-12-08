fn is_symbol(c: char) -> bool {
    !c.is_ascii_digit() && c != '.'
}

type Coordinate = [usize; 2];

#[derive(Debug, Clone, PartialEq)]
pub struct Grid {
    grid: Vec<char>,
    width: usize,
    height: usize,
}
impl Grid {
    pub fn new(s: &str) -> Grid {
        let mut line_iterator = s.lines();
        let first_line = line_iterator.next().unwrap();
        let width: usize = first_line.len();
        let mut grid: Vec<char> = first_line.chars().collect();
        let mut height: usize = 1;
        let mut next_line = line_iterator.next();
        while next_line.is_some() {
            grid.extend(next_line.unwrap().chars().collect::<Vec<_>>());
            height += 1;
            next_line = line_iterator.next();
        }

        Grid {
            grid,
            width,
            height,
        }
    }
    pub fn from_lines(lines_slice: &[&str]) -> Grid {
        let height: usize = lines_slice.len();
        let width: usize = lines_slice[0].len();
        // let mut grid: Vec<char> = lines_slice
        //     .iter()
        //     .map(|s| s.chars())
        //     .fold("".chars(), |a, b| a.chain(b))
        //     .collect();
        let mut grid: Vec<char> = Vec::new();
        for s in lines_slice {
            grid.append(&mut s.chars().collect::<Vec<char>>());
        }
        Grid { grid, width, height }
    }
    pub fn to_index(&self, coordinate: Coordinate) -> usize {
        coordinate[0] + self.width * coordinate[1]
    }
    pub fn to_coordinate(&self, index: usize) -> Coordinate {
        [index % self.width, index / self.width]
    }
    fn in_bounds(&self, coordinate: [isize; 2]) -> bool{
        coordinate[0] < self.width as isize
            && coordinate[0] >= 0
            && coordinate[1] < self.height as isize
            && coordinate[1] >= 0
    }

    fn get_surrounding_indicies_in_bounds(&self, index: usize) -> Vec<usize> {
        let temp_coordinate = self.to_coordinate(index);
        let mut coordinate = [temp_coordinate[0] as isize, temp_coordinate[1] as isize];
        let mut coordinates: [[isize; 2]; 8] = [
            [coordinate[0] + 1, coordinate[1]],
            [coordinate[0] + 1, coordinate[1] + 1],
            [coordinate[0], coordinate[1] + 1],
            [coordinate[0] - 1, coordinate[1] + 1],
            [coordinate[0] - 1, coordinate[1]],
            [coordinate[0] - 1, coordinate[1] - 1],
            [coordinate[0], coordinate[1] - 1],
            [coordinate[0] + 1, coordinate[1] - 1],

        ];
        let to_return = coordinates
            .into_iter()
            .filter(|c| self.in_bounds(*c))
            .map(|numbers| numbers.map(|n| n as usize))
            .map(|n| self.to_index(n))
            .collect();
        to_return
    
    }
    fn get_number_from_index_and_delete(&mut self, index: usize) -> usize {
        let coordinate = self.to_coordinate(index);
        let mut first_column: usize = 0;
        for i in (0..(coordinate[0])).rev() {
            if !self.grid[self.to_index([i, coordinate[1]])].is_ascii_digit() {
                first_column = i + 1;
                break;
            }
        }
        let mut last_column: usize = self.width - 1;
        for i in (coordinate[0])..(self.width) {
            if !self.grid[self.to_index([i, coordinate[1]])].is_ascii_digit() {
                last_column = i - 1;
                break;
            }
        }
        let first_index = self.to_index([first_column, coordinate[1]]);
        let last_index = self.to_index([last_column, coordinate[1]]);
        let mut char_vec: Vec<char> = Vec::new();
        for i in first_index..(last_index + 1) {
            char_vec.push(self.grid[i]);
            self.grid[i] = '.';
        }
        char_vec.iter().collect::<String>().parse::<usize>().unwrap()
        
    }
    pub fn print_grid(&self) {
        for i in 0..(self.height) {
            let mut to_print: String = String::new();
            for j in 0..(self.width) {
                to_print.push(self.grid[self.width*i + j]);
            }
            eprintln!("{}", to_print);
        }
    }
    pub fn get_part_numbers(&mut self) -> Vec<usize> {
        let mut to_return: Vec<usize> = Vec::new();
        for i in 0..(self.grid.len()) {
            if is_symbol(self.grid[i]) {
                let to_check = self.get_surrounding_indicies_in_bounds(i);
                for index in to_check {
                    if self.grid[index].is_ascii_digit() {
                        to_return.push(self.get_number_from_index_and_delete(index));
                    }
                }
            }
        }
        to_return
    }
}

pub fn find_part_numbers(s: &str) -> Vec<usize> {
    let mut grid = Grid::new(s);
    grid.get_part_numbers()
}

#[cfg(test)]
mod tests {
    use super::find_part_numbers;

    #[test]
    fn example() {
        let input = String::from(
"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
        );
        let part_numbers = find_part_numbers(&input);

        assert_eq!(4361, part_numbers.iter().sum::<usize>());
    }

    #[test]
    fn input_extract() {
        let input = String::from(
".......12.......935............184.720...243........589.652..........435..........483.............6...........................904...........
......*.....968*.....$............*........=..348...*..........986....*...................459....*........422................#......%482....
....291............612....290..........903........699......218*.......376............890....*.838...81......*.....138.../194................
..............156......$..*...891.&731....%..89...................523..........699....+...227......*.......225....=...........388....*......
................*...189..591.*................*.......783.....107..-...54.287..$................533.../..............909........&.603.424..."
        );
        let correct_part_numbers: Vec<usize> = vec![
            12,
            935,
            184,
            720,
            243,
            589,
            435,
            6,
            904,
            968,
            986,
            459,
            422,
            482,
            291,
            612,
            290,
            903,
            699,
            218,
            376,
            890,
            838,
            81,
            138,
            194,
            156,
            891,
            731,
            89,
            523,
            699,
            227,
            225,
            388,
            189,
            591,
            533,
            603,
            424,
        ];
        let correct_sum: usize = correct_part_numbers.iter().sum();
        let part_numbers = find_part_numbers(&input);
        let sum: usize = part_numbers.iter().sum();

        let mut sorted_part_numbers = part_numbers.clone();
        let mut sorted_correct_part_numbers = correct_part_numbers.clone();
        sorted_part_numbers.sort();
        sorted_correct_part_numbers.sort();

        assert_eq!(sorted_part_numbers, sorted_correct_part_numbers);
        assert_eq!(correct_sum, sum);
    }
}