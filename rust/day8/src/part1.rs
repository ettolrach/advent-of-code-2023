use std::{fmt::Display, str::FromStr};
use std::collections::HashMap;

type Element = [String; 2];
type Node = (String, Element);

#[derive(Debug)]
struct InstructionParseError;

enum Instruction {
    Left,
    Right,
}
impl FromStr for Instruction {
    type Err = InstructionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "L" => Ok(Instruction::Left),
            "R" => Ok(Instruction::Right),
            _ => Err(InstructionParseError),
        }
    }
}
impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Instruction::Left => "L",
            Instruction::Right => "R",
        })
    }
}

fn get_uppercase_string(s: &str) -> String {
    s.chars().filter(|c| c.is_ascii_uppercase()).collect()
}

fn parse_node_line(s: &str) -> Node {
    let mut s_equals_split = s.split('=');
    let name: String = s_equals_split
        .next()
        .unwrap()
        .trim()
        .to_string();
    let element: Element = s_equals_split
        .next()
        .unwrap()
        .trim()
        .split(',')
        .map(|s| get_uppercase_string(s))
        .collect::<Vec<String>>()
        .try_into()
        .unwrap();
    (name, element)
}

fn follow_instruction(instruction: &Instruction, element: &Element) -> String {
    match instruction {
        Instruction::Left => element[0].clone(),
        Instruction::Right => element[1].clone(),
    }
}

pub fn calculate_how_many_steps(input: &[&str]) -> usize {
    let mut hash_map: HashMap<String, Element>= HashMap::new();
    let mut input_lines = input.iter();
    let instructions: Vec<Instruction> = input_lines
        .next()
        .unwrap()
        .chars()
        .map(|c| Instruction::from_str(&c.to_string()).unwrap())
        .collect();
    input_lines.next();
    let nodes: Vec<Node> = input_lines.map(|s| parse_node_line(s)).collect();
    for node in nodes {
        hash_map.insert(node.0, node.1);
    }
    let mut current_node = String::from("AAA");
    let mut steps: usize = 0;
    let mut found = false;
    while !found {
        for instruction in &instructions {
            steps += 1;
            let current_element = hash_map.get(&current_node).unwrap();
            current_node = follow_instruction(instruction, current_element);
            if &current_node == "ZZZ" {
                found = true;
                break
            }
        }
    }
    steps
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;

    #[test]
    fn example1() {
        let input = String::from(
"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"
        );
        let steps = calculate_how_many_steps(&input.lines().collect::<Vec<&str>>()[..]);
        let expected_steps: usize = 2;
        assert_eq!(expected_steps, steps);
    }

    #[test]
    fn example2() {
        let input = String::from(
"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
        );
        let steps = calculate_how_many_steps(&input.lines().collect::<Vec<&str>>()[..]);
        let expected_steps: usize = 6;
        assert_eq!(expected_steps, steps);
    }
}