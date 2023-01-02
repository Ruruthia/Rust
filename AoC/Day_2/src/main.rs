// use std::io::{self, BufRead};

// fn main() -> io::Result<()> {
//     let mut lines = io::stdin().lock().lines();
//     let mut score = 0;
//     let mut current_val: i64 = 0;

//     while let Some(line) = lines.next() {
//         let last_input = line.unwrap();
//         let me = last_input.chars().nth(2).unwrap();
//         let opponent = last_input.chars().nth(0).unwrap();
//         if me == 'X' {
//             score = score + 1;
//             if opponent == 'A' {
//                 score = score + 3;
//             }
//             if opponent == 'C' {
//                 score = score + 6;
//             }
//         }
//         if me == 'Y' {
//             score = score + 2;
//             if opponent == 'B' {
//                 score = score + 3;
//             }
//             if opponent == 'A' {
//                 score = score + 6;
//             }
//         }
//         if me == 'Z' {
//             score = score + 3;
//             if opponent == 'C' {
//                 score = score + 3;
//             }
//             if opponent == 'B' {
//                 score = score + 6;
//             }
//         }
//     }
//     println!("{:?}", score);
//     Ok(())
// }

use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let mut lines = io::stdin().lock().lines();
    let mut score = 0;
    let mut current_val: i64 = 0;

    while let Some(line) = lines.next() {
        let last_input = line.unwrap();
        let result = last_input.chars().nth(2).unwrap();
        let opponent = last_input.chars().nth(0).unwrap();
        if result == 'X' {
            if opponent == 'A' {
                score = score + 3;
            }
            if opponent == 'B' {
                score = score + 1;
            }
            if opponent == 'C' {
                score = score + 2;
            }
        }
        if result == 'Y' {
            score = score + 3;
            if opponent == 'A' {
                score = score + 1;
            }
            if opponent == 'B' {
                score = score + 2;
            }
            if opponent == 'C' {
                score = score + 3;
            }
        }
        if result == 'Z' {
            score = score + 6;
            if opponent == 'A' {
                score = score + 2;
            }
            if opponent == 'B' {
                score = score + 3;
            }
            if opponent == 'C' {
                score = score + 1;
            }
        }
    }
    println!("{:?}", score);
    Ok(())
}