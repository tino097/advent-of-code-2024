use regex::Regex;
use std::error::Error;

struct Machine {
    button_a: (i64, i64), // (X, Y) movement for button A
    button_b: (i64, i64), // (X, Y) movement for button B
    prize: (i64, i64),    // Prize location (X, Y)
}

pub fn solve_part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let machines = parse_input(input)?;
    let mut total = 0;

    for (ba, bb, prize) in machines {
        let (x1, y1) = ba;
        let (x2, y2) = bb;
        let (mut px, mut py) = prize;

        // Check if solution exists
        if x2 * y1 - x1 * y2 == 0 || x1 == 0 {
            continue;
        }

        // Calculate n (presses of button B)
        let numerator = px * y1 - py * x1;
        let denominator = x2 * y1 - x1 * y2;

        if numerator % denominator != 0 {
            continue;
        }

        let n = numerator / denominator;

        // Calculate m (presses of button A)
        if (px - n * x2) % x1 != 0 {
            continue;
        }

        let m = (px - n * x2) / x1;

        // Calculate total tokens needed
        total += 3 * m + n;
    }

    Ok(total)
}

fn parse_input(input: &str) -> Result<Vec<((i64, i64), (i64, i64), (i64, i64))>, Box<dyn Error>> {
    let mut machines = Vec::new();
    let mut button_a = None;
    let mut button_b = None;
    let mut prize = None;

    let re_button_a = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)")?;
    let re_button_b = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)")?;
    let re_prize = Regex::new(r"Prize: X=(\d+), Y=(\d+)")?;

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        if let Some(caps) = re_button_a.captures(line) {
            button_a = Some((caps[1].parse::<i64>()?, caps[2].parse::<i64>()?));
        } else if let Some(caps) = re_button_b.captures(line) {
            button_b = Some((caps[1].parse::<i64>()?, caps[2].parse::<i64>()?));
        } else if let Some(caps) = re_prize.captures(line) {
            prize = Some((caps[1].parse::<i64>()?, caps[2].parse::<i64>()?));

            if let (Some(ba), Some(bb), Some(p)) = (button_a.take(), button_b.take(), prize.take())
            {
                machines.push((ba, bb, p));
            }
        }
    }

    Ok(machines)
}
