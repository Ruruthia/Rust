// use std::io::{self, BufRead, Read};
// fn main() -> io::Result<()> {

//     let mut score = 0;
//     let mut lines = include_str!("../input.txt").lines();
//     for line in lines {
//         let splitted_line: Vec<&str> = line.split(|c| c == ',' || c == '-').collect();
//         let first_start: i64 = splitted_line[0].parse().unwrap();
//         let first_end: i64 = splitted_line[1].parse().unwrap();
//         let second_start: i64 = splitted_line[2].parse().unwrap();
//         let second_end: i64 = splitted_line[3].parse().unwrap();
//         if (second_start >= first_start && second_end <= first_end) ||  (second_start <= first_start && second_end >= first_end) {
//             score += 1;
//         }
//     }

//     println!("{}", score);
//     Ok(())
// }


use std::io::{self, BufRead, Read};

fn main() -> io::Result<()> {

    let mut score = 0;
    let mut lines = include_str!("../input.txt").lines();
    for line in lines {
        let splitted_line: Vec<&str> = line.split(|c| c == ',' || c == '-').collect();
        let first_start: i64 = splitted_line[0].parse().unwrap();
        let first_end: i64 = splitted_line[1].parse().unwrap();
        let second_start: i64 = splitted_line[2].parse().unwrap();
        let second_end: i64 = splitted_line[3].parse().unwrap();
        if (second_start >= first_start && second_start <= first_end) ||  (second_start <= first_start && second_end >= first_start) {
            score += 1;
        }
    }

    println!("{}", score);
    Ok(())
}