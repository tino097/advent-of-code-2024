use std::error::Error;

pub fn solve_part2(input: &str) -> Result<i32, Box<dyn Error>> {
    let grid = parse_grid(input);
    let count = count_xmas(&grid);
    Ok(count)
}

type Grid = Vec<Vec<char>>;

fn parse_grid(input: &str) -> Grid {
    input
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn count_xmas(grid: &Grid) -> i32 {
    let patterns = vec![
        vec!["M.M", ".A.", "S.S"],
        vec!["S.M", ".A.", "S.M"],
        vec!["S.S", ".A.", "M.M"],
        vec!["M.S", ".A.", "M.S"],
    ];

    let mut count = 0;

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            for pattern in &patterns {
                if is_valid_pattern(x, y, pattern, grid) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn is_valid_pattern(x: usize, y: usize, pattern: &[&str], grid: &Grid) -> bool {
    for (dy, row) in pattern.iter().enumerate() {
        for (dx, ch) in row.chars().enumerate() {
            if ch == '.' {
                continue;
            }

            let ny = y + dy;
            let nx = x + dx;

            if ny >= grid.len() || nx >= grid[ny].len() || grid[ny][nx] != ch {
                return false;
            }
        }
    }
    true
}
