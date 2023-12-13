
pub fn difference_sequence(seq: &[isize]) -> Vec<isize> {
    if seq.len() < 2 {
        return seq.to_vec();
    }
    let mut to_return: Vec<isize> = Vec::new();
    for i in 1..(seq.len()) {
        to_return.push(seq[i] - seq[i - 1]);
    }
    to_return
}

pub fn all_equal<A>(slice: &[A]) -> bool
where
    A: PartialEq
{
    if slice.is_empty() {
        return false
    }
    slice.iter().all(|a| a == &slice[0])
}


pub fn next_in_sequence(seq: &[isize]) -> isize {
    let mut sequences: Vec<Vec<isize>> = vec![seq.to_vec()];
    while !all_equal(&sequences.last().unwrap()) {
        sequences.push(difference_sequence(&sequences.last().unwrap()));
    }
    let constant_number = sequences.last().unwrap()[0];
    (*sequences.last_mut().unwrap()).push(constant_number);
    for i in (1..(sequences.len())).rev() {
        let current_next_number = sequences[i].last().unwrap() + sequences[i - 1].last().unwrap();
        sequences[i - 1].push(current_next_number);
    }
    *sequences.first().unwrap().last().unwrap()
}

pub fn str_to_sequence(s: &str) -> Vec<isize> {
    s
        .split(' ')
        .map(|str_num| str_num.parse::<isize>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = String::from(
"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
        );
        let next_numbers: Vec<isize> = input
            .lines()
            .map(|s| s.split(' ').map(|subs| subs.parse::<isize>().unwrap()).collect::<Vec<_>>())
            .map(|nums| next_in_sequence(&nums[..]))
            .collect();
        let sum: isize = next_numbers.iter().sum();
        let expected_sum: isize = 114;
        assert_eq!(expected_sum, sum);
    }
}