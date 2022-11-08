fn summy(strng: &str) -> i32 {
    strng.split_whitespace().map(|d| d.parse::<i32>().unwrap()).sum()
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn sample_tests() {
        assert_eq!(summy("1 2 3"), 6);
        assert_eq!(summy("1 2 3 4"), 10);
        assert_eq!(summy("1 2 3 4 5"), 15);
        assert_eq!(summy("10 10"), 20);
        assert_eq!(summy("0 0"), 0);
    }

    #[test]
    fn my_test_1() {
        assert_eq!(summy("-1 1 0"), 0);
    }

    #[test]
    fn my_test_2() {
        assert_eq!(summy("1 -2 3 -4"), -2);
    }

    #[test]
    fn my_test_3() {
        assert_eq!(summy("1 200 3"), 204);
    }

    #[test]
    fn my_test_4() {
        assert_eq!(summy("1 2 3 4 5 6 7 8 9 10"), 55);
    }

    #[test]
    fn my_test_5() {
        assert_eq!(summy(""), 0);
    }
}