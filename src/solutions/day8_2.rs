use std::collections::{HashMap, HashSet};
use std::error::Error;

type Position = (i32, i32);

struct Grid {
    antennas: HashMap<char, Vec<Position>>,
    width: i32,
    height: i32,
}
pub fn solve_part2(input: &str) -> Result<i64, Box<dyn Error>> {
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
                    antennas
                        .entry(ch)
                        .or_insert(Vec::new())
                        .push((r as i32, c as i32));
                }
            }
            width = line.len() as i32;
            height += 1;
        }

        Grid {
            antennas,
            width,
            height,
        }
    }

    fn is_collinear(&self, p1: Position, p2: Position, p3: Position) -> bool {
        let (x1, y1) = p1;
        let (x2, y2) = p2;
        let (x3, y3) = p3;

        // Check if three points are collinear using cross product
        (y2 - y1) * (x3 - x1) == (y3 - y1) * (x2 - x1)
    }

    fn find_antinodes(&self) -> HashSet<Position> {
        let mut antinodes = HashSet::new();

        // For each frequency
        for positions in self.antennas.values() {
            if positions.len() < 2 {
                continue;
            }

            // Add antenna positions as antinodes if there are multiple antennas
            for &pos in positions {
                antinodes.insert(pos);
            }

            // Check every point in grid for collinearity
            for x in 0..self.height {
                for y in 0..self.width {
                    let point = (x, y);

                    // Count how many pairs of antennas this point is collinear with
                    for i in 0..positions.len() {
                        for j in i + 1..positions.len() {
                            if self.is_collinear(positions[i], positions[j], point) {
                                antinodes.insert(point);
                            }
                        }
                    }
                }
            }
        }

        antinodes
    }
}
