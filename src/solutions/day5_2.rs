use std::error::Error;

type Rule = (i32, i32);

pub fn solve_part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let mut rules: Vec<Rule> = Vec::new();
    let mut updates = Vec::new();
    let mut parsing = true;

    // Parse input
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
    
    for update in updates.iter_mut() {
        if !is_valid(update, &rules) {
            fix_update(update, &rules);

			let middle_idx = update.len() / 2;
			result += update[middle_idx] as i64;
            
        }
    }

    Ok(result)
}

fn is_valid(update: &[i32], rules: &[Rule]) -> bool {
    for &(first, after) in rules {
        if update.contains(&first) && update.contains(&after) {
            let pos_first = update.iter().position(|&x| x == first).unwrap();
            let pos_after = update.iter().position(|&x| x == after).unwrap();
            
            if pos_first > pos_after {
                return false;
            }
        }
    }
    true
}

fn fix_update(update: &mut Vec<i32>, rules: &[Rule]) {
    let mut changed = true;
    while changed {
        changed = false;
        for &(first, after) in rules {
            if update.contains(&first) && update.contains(&after) {
                let pos_first = update.iter().position(|&x| x == first).unwrap();
                let pos_after = update.iter().position(|&x| x == after).unwrap();
                
                if pos_first > pos_after {
                    // Swap elements to fix order
                    update.swap(pos_first, pos_after);
                    changed = true;
                }
            }
        }
    }
}