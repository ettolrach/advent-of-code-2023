use std::{fmt::Display, str::FromStr};
use std::collections::HashMap;

type Element = [String; 2];
type Node = (String, Element);

#[derive(Debug)]
pub struct InstructionParseError;

pub enum Instruction {
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

pub struct Traverser {
    pub current_node: String,
    pub finished: bool,
}
impl Traverser {
    pub fn new(starting_node: &str) -> Traverser {
        Traverser { current_node: String::from(starting_node), finished: false }
    }
    pub fn make_move(&mut self, instruction: &Instruction, hash_map: &HashMap<String, Element>) {
        self.current_node = match instruction {
            Instruction::Left => hash_map.get(&self.current_node).unwrap()[0].clone(),
            Instruction::Right => hash_map.get(&self.current_node).unwrap()[1].clone(),
        };
        self.finished = self.current_node.ends_with("Z");
    }
}

pub fn traversers_finished(traversers: &[Traverser]) -> bool {
    traversers.iter().all(|t| t.finished)
}

pub fn traversers_move(
    traversers: &mut [Traverser], 
    instruction: &Instruction,
    hash_map: &HashMap<String, Element>
)
{
    for t in traversers {
        t.make_move(instruction, hash_map)
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
    for node in nodes.clone() {
        hash_map.insert(node.0, node.1);
    }

    let starting_nodes: Vec<String> = nodes
        .iter()
        .map(|node| node.0.clone())
        .filter(|name| name.ends_with("A"))
        .collect();

    let mut traversers: Vec<Traverser> = Vec::new();
    for node in starting_nodes {
        traversers.push(Traverser::new(&node));
    }

    let mut steps: usize = 0;
    let mut found = false;
    while !found {
        for instruction in &instructions {
            steps += 1;
            traversers_move(&mut traversers, instruction, &hash_map);
            if traversers_finished(&traversers) {
                found = true;
                break;
            }
        }
    }
    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = String::from(
"LR

PPA = (PPB, XXX)
PPB = (XXX, PPZ)
PPZ = (PPB, XXX)
QQA = (QQB, XXX)
QQB = (QQC, QQC)
QQC = (QQZ, QQZ)
QQZ = (QQB, QQB)
XXX = (XXX, XXX)"
        );
        let steps = calculate_how_many_steps(&input.lines().collect::<Vec<&str>>()[..]);
        let expected_steps: usize = 6;
        assert_eq!(expected_steps, steps);
    }
}