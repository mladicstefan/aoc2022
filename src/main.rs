use std::collections::HashMap;
use std::fs;

fn main() {
    let movemap: HashMap<String, i32> = HashMap::from([
        ("A".to_string(), 1),
        ("B".to_string(), 2),
        ("C".to_string(), 3),
        ("X".to_string(), 1),
        ("Y".to_string(), 2),
        ("Z".to_string(), 3),
    ]);
    let strategy = fs::read_to_string("day2.txt").expect("Failed to read strategy...");

    let WIN_MULTIPLIER = 6;
    let DRAW_MULTIPLIER = 3;

    println!("{}", strategy);

    #[derive(Debug)]
    enum Outcome {
        Win,
        Loss,
        Draw,
    }
    let mut points = 0;

    for line in strategy.lines() {
        if line.is_empty() {
            break;
        } else {
            let mut moves: Vec<i32> = Vec::new();
            for char in line.trim().chars() {
                let mv_str = char.to_string();
                let mv = movemap.get(&mv_str);
                if let Some(num) = mv {
                    moves.push(*num);
                }
            }
            let result = ((moves[0] - moves[1]) % 3 + 3) % 3;
            let outcome = if result == 2 {
                Outcome::Win
            } else if result == 1 {
                Outcome::Loss
            } else {
                Outcome::Draw
            };

            match outcome {
                Outcome::Win => {
                    let round_score = moves[1] + WIN_MULTIPLIER;
                    points += round_score;
                    println!("Round score: {}", round_score);
                }
                Outcome::Loss => {
                    let round_score = moves[1];
                    points += round_score;
                    println!("Round score: {}", round_score);
                }
                Outcome::Draw => {
                    let round_score = moves[1] + DRAW_MULTIPLIER;
                    points += round_score;
                    println!("Round score: {}", round_score);
                }
            }
            println!(
                "Round: {} vs {}, outcome: {:?}",
                moves[0], moves[1], outcome
            );
            println!("{points}");
        }
    }
}
