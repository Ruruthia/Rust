use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let mut lines = io::stdin().lock().lines();
    let mut values = vec![];
    let mut current_val: i64 = 0;

    while let Some(line) = lines.next() {
        let last_input = line.unwrap();
        if last_input.len() == 0 {
            values.push(current_val);
            current_val = 0;
        }
        else {
            current_val = current_val + last_input.parse::<i64>().unwrap();
        }
    }
    values.sort();
    println!("{:?}", values.iter().rev().take(3).sum::<i64>());
    Ok(())
}
