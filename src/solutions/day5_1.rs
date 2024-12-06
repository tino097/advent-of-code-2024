use std::error::Error;

type Rule = (i32, i32);

pub fn solve_part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let mut rules: Vec<Rule> = Vec::new();
    let mut updates = Vec::new();
    let mut parsing = true;

    // Parse input (this part is correct)
    for line in input.lines() {
        if line.trim().is_empty() {
            parsing = false;
            continue;
        }

        if parsing {
            if let Some((x, y)) = line.split_once('|') {
                rules.push((
                    x.parse().unwrap(),
                    y.parse().unwrap()
                ));
            }
        } else {
            let update: Vec<i32> = line.split(',')
                .map(|n| n.parse().unwrap())
                .collect();
            updates.push(update);
        }
    }

    let mut result = 0;
    
    'update_loop: for update in updates.iter() {
        for &(first, next) in &rules {
            if update.contains(&first) && update.contains(&next) {
                let pos_before = update.iter().position(|&x| x == first).unwrap();
                let pos_after = update.iter().position(|&x| x == next).unwrap();
                
                if pos_before > pos_after {
                    continue 'update_loop; //goto
                }
            }
        }
        let middle_idx = update.len() / 2;
        result += update[middle_idx] as i64;
    }

    Ok(result)
}