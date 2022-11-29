fn descending_order(x: u64) -> u64 {
    let mut chars_vec = x.to_string().chars().collect::<Vec<char>>();
    chars_vec.sort_by(|a, b| b.cmp(a));
    chars_vec.iter().fold(0, |acc, elem| acc * 10 + (elem.to_digit(10).unwrap() as u64))
}