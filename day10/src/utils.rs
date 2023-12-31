use std::collections::VecDeque;

use itertools::Itertools;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Up,    // North
    Down,  // South
    Left,  // West
    Right, // East
}

impl Direction {
    pub const fn to_offset(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }

    pub const fn invert(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tile {
    Vertical,
    Horizontal,
    Bend(Direction, Direction),
    Ground,
    Start,
}

impl Tile {
    pub const fn parse_char(c: char) -> Self {
        match c {
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::Bend(Direction::Up, Direction::Right),
            'J' => Self::Bend(Direction::Up, Direction::Left),
            '7' => Self::Bend(Direction::Down, Direction::Left),
            'F' => Self::Bend(Direction::Down, Direction::Right),
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => unreachable!(),
        }
    }

    pub fn get_dirs(&self) -> Vec<Direction> {
        match self {
            Tile::Vertical => vec![Direction::Up, Direction::Down],
            Tile::Horizontal => vec![Direction::Left, Direction::Right],
            Tile::Bend(d1, d2) => vec![*d1, *d2],
            Tile::Ground => vec![],
            Tile::Start => vec![
                Direction::Left,
                Direction::Right,
                Direction::Up,
                Direction::Down,
            ],
        }
    }
}

#[derive(Clone, Debug)]
pub struct Tiles {
    tiles: Vec<Vec<Tile>>,
}

impl Tiles {
    pub fn parse<T: Iterator<Item = String>>(input: T) -> Self {
        let tiles = input
            .map(|line| line.trim().chars().map(Tile::parse_char).collect())
            .collect();

        Self { tiles }
    }

    pub fn neighbors(&self, (i, j): (usize, usize)) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.tiles[i][j]
            .get_dirs()
            .into_iter()
            .filter_map(move |d| {
                let (di, dj) = d.to_offset();

                let neighbor = ((i as isize + di) as usize, (j as isize + dj) as usize);

                if neighbor.0 >= self.tiles.len() || neighbor.1 >= self.tiles[0].len() {
                    return None;
                }

                let neighbor_tile = self.tiles[neighbor.0][neighbor.1];

                if neighbor_tile.get_dirs().contains(&d.invert()) {
                    Some(neighbor)
                } else {
                    None
                }
            })
    }

    fn get_start_point(&self) -> (usize, usize) {
        for (i, outer) in self.tiles.iter().enumerate() {
            for (j, el) in outer.iter().enumerate() {
                if let Tile::Start = el {
                    return (i, j);
                }
            }
        }

        unreachable!()
    }

    pub fn get_loop(&self) -> Vec<(usize, usize)> {
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        let mut visited = vec![vec![false; self.tiles[0].len()]; self.tiles.len()];

        let mut vertices: Vec<(usize, usize)> = vec![];

        queue.push_back(self.get_start_point());

        while let Some((i, j)) = queue.pop_back() {
            if visited[i][j] {
                continue;
            }
            visited[i][j] = true;

            vertices.push((i, j));
            queue.extend(&mut self.neighbors((i, j)));
        }

        vertices
    }

    pub fn get_max_dist(&self) -> usize {
        let len = self.get_loop().len();

        (len + 1) / 2
    }

    pub fn get_area(&self) -> usize {
        let vertices = self.get_loop();

        let surveyors_sum = vertices
            .iter()
            .circular_tuple_windows()
            .map(|((ci, cj), (ni, nj))| {
                let a = ci * nj;
                let b = cj * ni;

                a as f64 - b as f64
            })
            .sum::<f64>()
            .abs()
            / 2.0;

        // prob should still use float
        surveyors_sum as usize + 1 - vertices.len() / 2
    }
}
