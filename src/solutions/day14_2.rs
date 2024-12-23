use std::error::Error;
use std::collections::{HashSet, HashMap};

#[derive(Debug, Clone)]
struct Robot {
    pos: (i32, i32),
    vel: (i32, i32),
}

pub fn solve_part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let mut robots = parse_input(input)?;
    let width = 101;
    let height = 103;
    let mut step = 0;

    loop {
        // Move robots
        for robot in &mut robots {
            robot.pos.0 = (robot.pos.0 + robot.vel.0).rem_euclid(width);
            robot.pos.1 = (robot.pos.1 + robot.vel.1).rem_euclid(height);
        }
        step += 1;

        // Find clusters
        let clusters = find_clusters(&robots);
        
        // Check for large clusters (potential messages)
        for cluster in clusters.values() {
            if cluster.len() >= robots.len() / 5 {
                print_grid(&robots, width, height);
                if cluster.len() >= robots.len() / 3 {
                    return Ok(step);
                }
            }
        }
    }
}

fn find_clusters(robots: &[Robot]) -> HashMap<i32, HashSet<(i32, i32)>> {
    let mut clusters = HashMap::new();
    let mut seen = HashSet::new();
    let robot_positions: HashSet<(i32, i32)> = robots.iter()
        .map(|r| r.pos)
        .collect();
    
    let mut cluster_id = 0;
    for robot in robots {
        if seen.contains(&robot.pos) {
            continue;
        }

        cluster_id += 1;
        let mut cluster = HashSet::new();
        let mut queue = vec![robot.pos];

        while let Some((x, y)) = queue.pop() {
            if seen.contains(&(x, y)) {
                continue;
            }

            seen.insert((x, y));
            cluster.insert((x, y));

            // Check neighbors
            for &(dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let next_pos = (x + dx, y + dy);
                if robot_positions.contains(&next_pos) {
                    queue.push(next_pos);
                }
            }
        }

        clusters.insert(cluster_id, cluster);
    }

    clusters
}

fn print_grid(robots: &[Robot], width: i32, height: i32) {
    let positions: HashSet<(i32, i32)> = robots.iter()
        .map(|r| r.pos)
        .collect();

    for y in 0..height {
        for x in 0..width {
            if positions.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
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