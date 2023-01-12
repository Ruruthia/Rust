fn encode(msg: String, n: i32) -> Vec<i32> {
    let word: Vec<i32> = msg.chars().map(|c| c as i32 - 96).collect();
    let key: Vec<i32> = n.to_string().chars().map(|c| c.to_digit(10).unwrap() as i32).cycle().take(msg.len()).collect();
    (0..word.len()).map(|i| word[i] + key[i]).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn fixed_tests() {
        assert_eq!(encode("scout".to_string(), 1939), vec![20, 12, 18, 30, 21]);
        assert_eq!(encode("masterpiece".to_string(), 1939), vec![14, 10, 22, 29, 6, 27, 19, 18, 6, 12, 8]);
    }
}