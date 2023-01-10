fn last_digit(list: &[u64]) -> u64 {
    if list.len() == 0 {
        return 1;
    }
    list.iter().copied().rev().reduce(|acc, val| ((if val < 20 {val} else {val % 20 + 20}).pow(if acc < 4 {acc as u32} else {(acc % 4 + 4) as u32}))).unwrap() % 10
}

mod tests {
    use super::last_digit;
    
    fn reference_solution() {

    }
    
    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";
    
    fn dotest(v: &[u64], expected: u64) {
        assert_eq!(last_digit(v), expected, "{ERR_MSG} with list = {v:?}")
    }

    #[test]
    fn fixed_tests() {
        for (a, b) in  [
                        (vec![], 1),
                        (vec![0, 0], 1),
                        (vec![0, 0, 0], 0),
                        (vec![1, 2], 1),
                        (vec![3, 4, 5], 1),
                        (vec![4, 3, 6], 4),
                        (vec![7, 6, 21], 1),
                        (vec![12, 30, 21], 6),
                        (vec![2, 2, 2, 0], 4),
                        (vec![2, 2, 101, 2], 6),
                        (vec![937640, 767456, 981242], 0),
                        (vec![123232, 694022, 140249], 6),
                        (vec![499942, 898102, 846073], 6)
                      ]  {
            dotest(&a, b);
        }
    }
}
