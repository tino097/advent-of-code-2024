use std::error::Error;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Robot {
    pos: (i32, i32),  // (x, y)
    vel: (i32, i32),  // (vx, vy)
}

pub fn solve_part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let mut robots = parse_input(input)?;
    let width = 101;
    let height = 103;
    
    // Simulate 100 seconds
    for _ in 0..100 {
        for robot in &mut robots {
            // Update position
            robot.pos.0 = (robot.pos.0 + robot.vel.0).rem_euclid(width);
            robot.pos.1 = (robot.pos.1 + robot.vel.1).rem_euclid(height);
        }
    }

    // Count robots in each quadrant
    let mut quadrants = vec![0, 0, 0, 0];  // TL, TR, BL, BR
    let mid_x = width / 2;
    let mid_y = height / 2;

    for robot in robots {
        if robot.pos.0 == mid_x || robot.pos.1 == mid_y {
            continue;  // Skip robots on center lines
        }

        let quadrant = match (robot.pos.0 < mid_x, robot.pos.1 < mid_y) {
            (true, true) => 0,   // Top Left
            (false, true) => 1,  // Top Right
            (true, false) => 2,  // Bottom Left
            (false, false) => 3, // Bottom Right
        };
        
        quadrants[quadrant] += 1;
    }

    // Calculate safety factor
    let safety_factor = quadrants.iter().product::<i32>();
    println!("Quadrants: {:?}", quadrants);
    Ok(safety_factor as i64)
}

fn parse_input(input: &str) -> Result<Vec<Robot>, Box<dyn Error>> {
    input.lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let pos = parse_coords(parts[0].trim_start_matches("p="))?;
            let vel = parse_coords(parts[1].trim_start_matches("v="))?;
            
            Ok(Robot { pos, vel })
        })
        .collect()
}

fn parse_coords(s: &str) -> Result<(i32, i32), Box<dyn Error>> {
    let coords: Vec<i32> = s.split(',')
        .map(|n| n.parse::<i32>())
        .collect::<Result<Vec<i32>, _>>()?;
    
    Ok((coords[0], coords[1]))
}