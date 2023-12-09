use crate::part1::Grid;

const GEAR: char = '*';

pub fn get_gear_ratios_and_delete(grid: &mut Grid) -> Vec<usize> {
	let mut to_return = Vec::new();
	for i in 0..(grid.grid.len()) {
		if grid.grid[i] == GEAR {
			let to_check = grid.get_surrounding_indicies_in_bounds(i);
			let mut potential_parts: Vec<usize> = Vec::new();
			for index in to_check {
				if grid.grid[index].is_ascii_digit() {
					potential_parts.push(grid.get_number_from_index_and_delete(index));
				}
			}
			if potential_parts.len() == 2 {
				to_return.push(potential_parts[0] * potential_parts[1]);
			}
		}
	}
	to_return
}

#[cfg(test)]
mod tests {
    use crate::part1::Grid;

    use super::get_gear_ratios_and_delete;

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
		let mut grid = Grid::new(&input);
		let gear_ratios = get_gear_ratios_and_delete(&mut grid);
		let sum: usize = gear_ratios.iter().sum();
		let expected_sum: usize = 467835;
		assert_eq!(expected_sum, sum);
	}
}
