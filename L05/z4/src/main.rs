fn expanded_form(n: u64) -> String {
    let mut res : Vec<String> = n.to_string().chars().rev()
    .enumerate().map(|(i,d)| ((d.to_digit(10).unwrap() as u64 *  (10_i64.pow(i as u32) as u64))).to_string()).collect::<Vec<String>>();
    res.reverse();
    res.retain(|x| *x != "0");
    res.join(" + ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(expanded_form(12), "10 + 2");
        assert_eq!(expanded_form(42), "40 + 2");
        assert_eq!(expanded_form(70304), "70000 + 300 + 4");
    }

    #[test]
    fn my_test_1() {
        assert_eq!(expanded_form(0), "");
    }

    #[test]
    fn my_test_2() {
        assert_eq!(expanded_form(123456), "100000 + 20000 + 3000 + 400 + 50 + 6");
    }

    #[test]
    fn my_test_3() {
        assert_eq!(expanded_form(1010101010), "1000000000 + 10000000 + 100000 + 1000 + 10");
    }

    #[test]
    fn my_test_4() {
        assert_eq!(expanded_form(1000000001), "1000000000 + 1");
    }

    #[test]
    fn my_test_5() {
        assert_eq!(expanded_form(214702713), "200000000 + 10000000 + 4000000 + 700000 + 2000 + 700 + 10 + 3");
    }
}