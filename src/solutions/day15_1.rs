use std::error::Error;
use std::collections::HashMap;

type Position = (i32, i32);
type Warehouse = HashMap<Position, char>;

#[derive(Debug)]
struct Map {
    warehouse: Warehouse,
    robot_pos: Position,
}

fn parse_input(input: &str) -> (Vec<&str>, String) {
    let mut warehouse = Vec::new();
    let mut instructions = String::new();

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let first_char = line.trim().chars().next().unwrap();
        if "><v^".contains(first_char) {
            instructions.push_str(line.trim());
        } else {
            warehouse.push(line.trim());
        }
    }

    (warehouse, instructions)
}

fn warehouse_map(warehouse: &[&str]) -> Result<Map, Box<dyn Error>> {
    let mut wm = HashMap::new();
    let mut robot_position = None;

    for (y, row) in warehouse.iter().enumerate() {
        for (x, c) in row.chars().enumerate() {
            if c == '@' {
                if robot_position.is_some() {
                    return Err("Multiple robots found".into());
                }
                robot_position = Some((x as i32, y as i32));
            } else if c == '#' || c == 'O' || c == '[' || c == ']' {
                wm.insert((x as i32, y as i32), c);
            }
        }
    }

    Ok(Map {
        warehouse: wm,
        robot_pos: robot_position.ok_or("No robot found")?
    })
}

fn peek_ahead(pos: Position, direction: Position, wm: &Warehouse) -> (bool, Vec<Position>) {
    let (x, y) = pos;
    let (dx, dy) = direction;
    let mut xx = x + dx;
    let mut yy = y + dy;

    if !wm.contains_key(&(xx, yy)) {
        return (true, vec![]);
    }

    if wm.get(&(xx, yy)) == Some(&'O') {
        let mut boxes = Vec::new();
        while wm.contains_key(&(xx, yy)) {
            if wm.get(&(xx, yy)) == Some(&'O') {
                boxes.push((xx, yy));
            } else if wm.get(&(xx, yy)) == Some(&'#') {
                return (false, vec![]);
            }
            xx += dx;
            yy += dy;
        }
        return (true, boxes);
    }

    (false, vec![])
}

pub fn solve_part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let (warehouse, instructions) = parse_input(input);
    let mut map = warehouse_map(&warehouse)?;
    
    let directions: HashMap<char, Position> = [
        ('>', (1, 0)),
        ('<', (-1, 0)),
        ('^', (0, -1)),
        ('v', (0, 1)),
    ].iter().cloned().collect();

    for instruction in instructions.chars() {
        let direction = *directions.get(&instruction)
            .ok_or("Invalid direction")?;
            
        let (can_move, boxes) = peek_ahead(map.robot_pos, direction, &map.warehouse);
        
        if can_move {
            // Move boxes
            let (dx, dy) = direction;
            let mut moved_boxes = Vec::new();
            
            for pos in boxes {
                moved_boxes.push((pos.0 + dx, pos.1 + dy));
                map.warehouse.remove(&pos);
            }
            
            for pos in moved_boxes {
                map.warehouse.insert(pos, 'O');
            }
            
            // Move robot
            map.robot_pos = (map.robot_pos.0 + dx, map.robot_pos.1 + dy);
        }
    }

    // Calculate total
    let total: i64 = map.warehouse.iter()
        .filter(|(_, &c)| c == 'O')
        .map(|((x, y), _)| 100 * (*y as i64) + (*x as i64))
        .sum();

    Ok(total)
}