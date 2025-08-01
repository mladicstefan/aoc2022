use std::fs;

fn main() {
    let content = fs::read_to_string("day1.txt").expect("Failed Reason");

    let mut numbers: Vec<Option<i32>> = Vec::new();

    for line in content.lines() {
        if line.trim().is_empty() {
            numbers.push(None);
        } else {
            let num = line.trim().parse::<i32>().unwrap();
            numbers.push(Some(num));
        }
    }
    let mut current: i32 = 0;
    let mut max: i32 = 0;

    for num in numbers.iter() {
        match num {
            Some(num) => {
                current += num;
            }
            None => {
                if current >= max {
                    max = current;
                }
                current = 0;
            }
        }
    }
    println!("max:{}", max);
}
