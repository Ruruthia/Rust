fn solution(num: i32) -> i32 {
    let multiples_of_3: i32 = (0..num).step_by(3).sum();
    let multiples_of_5: i32 = (0..num).step_by(5).sum();  
    let multiples_of_15: i32 = (0..num).step_by(15).sum();
      multiples_of_3 + multiples_of_5 - multiples_of_15
}

mod tests {
    use super::solution;
    
    #[test]
    fn sample_tests() {
      // assertion(expected, input);
      assertion(23, 10);
      assertion(33, 11);
      assertion(225, 33);
      assertion(8, 6);
      assertion(3420, 123);
      assertion(543, 50);
      assertion(25719750, 10500);
    }
    
    fn assertion(expected : i32, input : i32) {
        let actual = solution(input);
        
        assert!(
            expected == actual,
            "\nTest failed!\n expected: {}\n actual: {}\n input: {}\n", expected, actual, input
        );
    }

    #[test]
    fn my_test_1() {
      assertion(0, 0);
    }

    #[test]
    fn my_test_2() {
      assertion(3, 5);
    }

    #[test]
    fn my_test_3() {
      assertion(3668, 126);
    }

    #[test]
    fn my_test_4() {    
        assertion(91961744, 19852);
    }

    #[test]
    fn my_test_5() {
      assertion(574883, 1570);
    }
}