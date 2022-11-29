fn even_numbers(array: &Vec<i32>, number: usize) -> Vec<i32> {
    let even = array.iter().copied().filter(|&i| i % 2 == 0).collect::<Vec<_>>();
    even[even.len() - number..].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn sample_tests() {
        assert_eq!(even_numbers(&vec!(1, 2, 3, 4, 5, 6, 7, 8, 9), 3), vec!(4, 6, 8));
        assert_eq!(even_numbers(&vec!(-22, 5, 3, 11, 26, -6, -7, -8, -9, -8, 26), 2), vec!(-8, 26));
        assert_eq!(even_numbers(&vec!(6, -25, 3, 7, 5, 5, 7, -3, 23), 1), vec!(6));
    }

    #[test]
    fn my_test_1() {
        assert_eq!(even_numbers(&vec!(0), 1), vec!(0));
    }

    #[test]
    fn my_test_2() {
        assert_eq!(even_numbers(&vec!(), 0), vec!());
    }   

    #[test]
    fn my_test_3() {
        assert_eq!(even_numbers(&vec!(1, 14, 28, 12, 58, -12, 4, 11), 3), vec!(58, -12, 4));
    }   

    #[test]
    fn my_test_4() {
        assert_eq!(even_numbers(&vec!(73, 4714, 4234, 134, 14, 56, 73, 26, 73), 5), vec!(4234, 134, 14, 56, 26));
    }   

    #[test]
    fn my_test_5() {
        assert_eq!(even_numbers(&vec!(794619, 314531, -144, 134, -14, 1415, -13515, 13, 52, 2, -52), 4), vec!(-14, 52, 2, -52));
    }      
}