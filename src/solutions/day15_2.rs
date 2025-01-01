use std::error::Error;
use std::collections::HashMap;

type Position = (i32, i32);
type Warehouse = HashMap<Position, char>;

#[derive(Debug)]
struct Map {
    warehouse: Warehouse,
    robot_pos: Position,
}

fn widen_warehouse(warehouse: &[&str]) -> Vec<String> {
    let mut wide = Vec::new();
    
    for row in warehouse {
        let mut wide_row = String::new();
        for c in row.chars() {
            match c {
                '#' => wide_row.push_str("##"),
                '@' => wide_row.push_str("@."),
                'O' => wide_row.push_str("[]"),
                '.' => wide_row.push_str(".."),
                _ => wide_row.push_str("..")
            }
        }
        wide.push(wide_row);
    }
    
    wide
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

fn warehouse_map(warehouse: &[String]) -> Result<Map, Box<dyn Error>> {
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

fn peek_ahead_wide(pos: Position, direction: Position, wm: &Warehouse) -> (bool, Vec<Position>) {
    let (x, y) = pos;
    let (dx, dy) = direction;
    let (xx, yy) = (x + dx, y + dy);

    if !wm.contains_key(&(xx, yy)) {
        return (true, vec![]);
    }

    if dx != 0 && dy == 0 {  // Horizontal movement
        if wm.get(&(xx, yy)).map_or(false, |&c| c == '[' || c == ']') {
            let mut boxes = Vec::new();
            let mut current = (xx, yy);
            
            while wm.contains_key(&current) {
                if wm.get(&current).map_or(false, |&c| c == '[' || c == ']') {
                    boxes.push(current);
                } else if wm.get(&current) == Some(&'#') {
                    return (false, vec![]);
                }
                current.0 += dx;
            }
            return (true, boxes);
        }
        return (false, vec![]);
    } else {  // Vertical movement
        if wm.get(&(xx, yy)) == Some(&'#') {
            return (false, vec![]);
        }

        let mut boxes = Vec::new();
        let mut topmost_boxes = vec![];
        
        if wm.get(&(xx, yy)) == Some(&'[') {
            topmost_boxes.push((xx, yy));
            topmost_boxes.push((xx + 1, yy));
            boxes.extend(&topmost_boxes);
        } else if wm.get(&(xx, yy)) == Some(&']') {
            topmost_boxes.push((xx, yy));
            topmost_boxes.push((xx - 1, yy));
            boxes.extend(&topmost_boxes);
        }

        while !topmost_boxes.is_empty() {
            let mut new_topmost = Vec::new();
            
            for &(bx, by) in &topmost_boxes {
                let next = (bx + dx, by + dy);
                if !wm.contains_key(&next) {
                    continue;
                }
                if wm.get(&next) == Some(&'#') {
                    return (false, vec![]);
                }
                if wm.get(&next) == Some(&'[') {
                    new_topmost.push(next);
                    new_topmost.push((next.0 + 1, next.1));
                } else if wm.get(&next) == Some(&']') {
                    new_topmost.push(next);
                    new_topmost.push((next.0 - 1, next.1));
                }
            }
            
            boxes.extend(&new_topmost);
            topmost_boxes = new_topmost;
        }

        return (true, boxes);
    }
}

pub fn solve_part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let (warehouse, instructions) = parse_input(input);
    let wide_warehouse = widen_warehouse(&warehouse);
    let mut game = warehouse_map(&wide_warehouse)?;
    
    let directions: HashMap<char, Position> = [
        ('>', (1, 0)),
        ('<', (-1, 0)),
        ('^', (0, -1)),
        ('v', (0, 1)),
    ].iter().cloned().collect();

    for instruction in instructions.chars() {
        let direction = *directions.get(&instruction).unwrap();
        let (can_move, boxes) = peek_ahead_wide(game.robot_pos, direction, &game.warehouse);
        
        if can_move {
            let (dx, dy) = direction;
            let mut moved = Vec::new();
            
            for pos in boxes {
                if let Some(c) = game.warehouse.remove(&pos) {
                    moved.push((pos.0 + dx, pos.1 + dy, c));
                }
            }
            
            for (x, y, c) in moved {
                game.warehouse.insert((x, y), c);
            }
            
            game.robot_pos = (game.robot_pos.0 + dx, game.robot_pos.1 + dy);
        }
    }

    let total: i64 = game.warehouse.iter()
        .filter(|(_, &c)| c == '[')
        .map(|((x, y), _)| 100 * (*y as i64) + (*x as i64))
        .sum();

    Ok(total)
}