use std::error::Error;
use std::collections::{HashMap, HashSet};

type Position = (i32, i32);

struct Grid {
    antennas: HashMap<char, Vec<Position>>,
    width: i32,
    height: i32,
}
pub fn solve_part1(input: &str) -> Result<i64, Box<dyn Error>> {
	let grid = Grid::parse(input);
	let antinodes = grid.find_antinodes();
	Ok(antinodes.len() as i64)
}

impl Grid {
    fn parse(input: &str) -> Self {
        let mut antennas = HashMap::new();
        let mut height = 0;
        let mut width = 0;

        for (r, line) in input.lines().enumerate() {
            for (c, ch) in line.chars().enumerate() {
                if ch != '.' {
                    antennas.entry(ch)
                        .or_insert(Vec::new())
                        .push((r as i32, c as i32));
                }
            }
            width = line.len() as i32;
            height += 1;
        }

        Grid { antennas, width, height }
    }


fn compute_antinodes(&self, p1: Position, p2: Position) -> HashSet<Position> {
	let mut antinodes = HashSet::new();
	let dx = p2.0 - p1.0;
	let dy = p2.1 - p1.1;

	for direction in [-1, 1].iter() {
		let mut current = if *direction == -1 { p1 } else { p2 };
		let mut next = (
			current.0 + dx * direction,
			current.1 + dy * direction
		);
		
		if self.in_bounds(next) {
			antinodes.insert(next);
		}
	}

	antinodes
}

fn find_antinodes(&self) -> HashSet<Position> {
	let mut antinodes = HashSet::new();

	for positions in self.antennas.values() {
		for i in 0..positions.len() {
			for j in i+1..positions.len() {
				antinodes.extend(
					self.compute_antinodes(positions[i], positions[j])
				);
			}
		}
	}

	antinodes
}

fn in_bounds(&self, pos: Position) -> bool {
	pos.0 >= 0 && pos.0 < self.height && 
	pos.1 >= 0 && pos.1 < self.width
}
}

