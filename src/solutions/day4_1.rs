use std::error::Error;

pub fn solve_part1(input: &str) -> Result<i32, Box<dyn Error>> {
   let grid = parse_grid(input);
   let count = count_xmas(&grid);
   Ok(count)
}

type Grid = Vec<Vec<char>>;

fn parse_grid(input: &str) -> Grid {
   input.lines()
       .map(|line| line.chars().collect())
       .collect()
}

fn count_xmas(grid: &Grid) -> i32 {
   let directions = [
       (0, 1),   // right
       (1, 0),   // down
       (1, 1),   // diagonal down-right
       (-1, 1),  // diagonal up-right
       (0, -1),  // left
       (-1, 0),  // up
       (-1, -1), // diagonal up-left
       (1, -1)   // diagonal down-left
   ];

   let mut count = 0;
   let rows = grid.len() as i32;
   let cols = grid[0].len() as i32;

   for r in 0..rows {
       for c in 0..cols {
           for &(dr, dc) in &directions {
               if check_xmas(grid, r, c, dr, dc, rows, cols) {
                   count += 1;
               }
           }
       }
   }
   count
}

fn check_xmas(grid: &Grid, start_r: i32, start_c: i32, dr: i32, dc: i32, rows: i32, cols: i32) -> bool {
    let target = ['X', 'M', 'A', 'S'];
    
    for (i, &target_char) in target.iter().enumerate() {
        let r = start_r + i as i32 * dr;
        let c = start_c + i as i32 * dc;

        if r < 0 || r >= rows || c < 0 || c >= cols {
            return false;
        }

        if grid[r as usize][c as usize] != target_char {
            return false;
        }
    }
    true
}