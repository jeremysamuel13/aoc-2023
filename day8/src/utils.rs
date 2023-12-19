use std::collections::HashMap;

use itertools::FoldWhile;
use itertools::Itertools;
use num::Integer;

type NodeId = String;

#[derive(Debug, Clone)]
pub struct Map {
    pub instructions: Vec<Instruction>,
    pub graph: Graph,
}

impl Map {
    pub fn parse_from_iter<T: Iterator<Item = String>>(mut input: T) -> Self {
        let instructions = input
            .next()
            .expect("Failed to get instructions")
            .chars()
            .map(Instruction::from)
            .collect();

        let graph = input
            .skip(1)
            .map(|s| {
                let s = s.trim();
                let start = s[0..3].to_string();
                let left = s[7..10].to_string();
                let right = s[12..15].to_string();

                (start, AdjacentNodes { left, right })
            })
            .collect();

        Self {
            instructions,
            graph,
        }
    }
}

impl Map {
    pub fn steps(&self, start: String) -> usize {
        self.instructions
            .iter()
            .cycle()
            .fold_while((0, start), |(count, node), i| {
                if node.ends_with('Z') {
                    FoldWhile::Done((count, node))
                } else {
                    FoldWhile::Continue((count + 1, self.graph.get_with_instruction(&node, i)))
                }
            })
            .into_inner()
            .0
    }
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Left,
    Right,
}

impl From<char> for Instruction {
    fn from(value: char) -> Self {
        match value {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AdjacentNodes {
    pub left: NodeId,
    pub right: NodeId,
}

#[derive(Debug, Clone)]
pub struct Graph {
    adjacency_map: HashMap<NodeId, AdjacentNodes>,
}

impl Graph {
    pub fn get_with_instruction(&self, node: &NodeId, instruction: &Instruction) -> NodeId {
        let adj = &self.adjacency_map[node];
        match instruction {
            Instruction::Left => adj.left.clone(),
            Instruction::Right => adj.right.clone(),
        }
    }

    pub fn ghost_start(&self) -> impl Iterator<Item = &String> {
        self.adjacency_map.keys().filter(|node| node.ends_with('A'))
    }
}

impl FromIterator<(NodeId, AdjacentNodes)> for Graph {
    fn from_iter<T: IntoIterator<Item = (NodeId, AdjacentNodes)>>(iter: T) -> Self {
        Self {
            adjacency_map: iter.into_iter().collect(),
        }
    }
}

pub trait NumExt: Iterator<Item = usize> + Sized {
    fn lcm(self) -> Option<usize> {
        self.reduce(|acc, x| match (acc, x) {
            (0, 0) => 0,
            (a, b) => a * (b / a.gcd(&b)),
        })
    }
}

impl<I> NumExt for I where I: Iterator<Item = usize> {}
