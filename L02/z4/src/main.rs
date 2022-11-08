use itertools::Itertools;

fn longest(a1: &str, a2: &str) -> String {
    let binding = a1.to_owned() + a2;
    let all_chars = binding.chars();
    all_chars.sorted().dedup().collect()
}

#[cfg(test)]
    mod tests {
    use super::*;
   
    fn testing(s1: &str, s2: &str, exp: &str) -> () {
        println!("s1:{:?} s2:{:?}", s1, s2);
        println!("{:?} {:?}", longest(s1, s2), exp);
        println!("{}", longest(s1, s2) == exp);
        assert_eq!(&longest(s1, s2), exp)
    }

    #[test]
    fn basic_tests() {
        testing("aretheyhere", "yestheyarehere", "aehrsty");
        testing("loopingisfunbutdangerous", "lessdangerousthancoding", "abcdefghilnoprstu");
        
    }

    #[test]
    fn my_test_1() {
        testing("aretheyhere", "yestheyarehere", "aehrsty");
    }

    #[test]
    fn my_test_2() {
        testing("", "", "");
    }

    #[test]
    fn my_test_3() {
        testing("aretheyhere", "", "aehrty");
    }

    #[test]
    fn my_test_4() {
        testing("", "aretheyhere", "aehrty");
    }

    #[test]
    fn my_test_5() {
        testing("abcż", "abcź", "abcźż");
    }
}

