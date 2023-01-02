fn to_priority(c: char) -> usize {
    match c {
        'a'..='z' => c as usize - 96,
        'A'..='Z' => c as usize - 64 + 26,
        _ => 0,
    }
}

use std::io::{self, BufRead, Read};
use std::fs::File;
use itertools::Chunks;

// fn main() -> io::Result<()> {
//     let lines = io::stdin().lock().lines();
//     let result: usize = 
//         lines.map(
//             |line| {
//                 let len = line.as_ref().unwrap().len();
//                 let mut seen = [0; 53];
//                 for (i, c) in line.unwrap().chars().enumerate() {
//                     if i < len / 2 {
//                         seen[to_priority(c)] = 1;
//                     } else {
//                         let priority = to_priority(c);
//                         if seen[priority] != 0 {
//                             return priority;
//                         }
//                     }
//                 }
//                 0
//             })
//             .sum();
//     println!("{:?}", result);
//     Ok(())
// }

fn filename_to_string(s: &str) -> io::Result<String> {
    let mut file = File::open(s)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}

fn main() -> io::Result<()> {

    let mut score = 0;
    let mut lines = include_str!("../input.txt").lines();
    while let (Some(first), Some(second), Some(third)) = (lines.next(), lines.next(), lines.next()) {
        let result = first.chars().filter(|c| {
            second.chars().position(|x| x == *c).is_some() && third.chars().position(|y| y == *c).is_some()
        }).map(|c| c as u8).next().unwrap();
            score += u32::from(match result {
                65..=90 => result - 38,
                97..=122 => result - 96,
                 _ => unreachable!()
            });
    }
    println!("{}", score);
    Ok(())
}
