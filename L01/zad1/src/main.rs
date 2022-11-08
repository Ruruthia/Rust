// NOT OPTIMAL VERSION:
// fn string_to_number(number_string: &str) -> i32 {
//     let mut number_int: i32 = 0;
//     let mut decimals: u64 = 1;
//     for ch in number_string.chars().rev(){
//         if ch == '-' {
//             number_int *= -1;
//         }
//         else {
//             let digit = ch.to_digit(10).unwrap_or(0);
//             number_int += ((decimals as u32) * digit) as i32;
//             decimals *= 10;
//         }
//     }
//     return number_int
// }

// OPTIMAL VERSION:
fn string_to_number(number_string: &str) -> i32 {
  number_string.parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::string_to_number;
    use rand::prelude::*;

    #[test]
    fn returns_expected() {
      assert_eq!(string_to_number("1234"), 1234);
      assert_eq!(string_to_number("605"), 605);
      assert_eq!(string_to_number("1405"), 1405);
      assert_eq!(string_to_number("-7"), -7);
    }

    #[test]
    fn works_on_random() {
        let mut rng = thread_rng();
        for _ in 0..5 {
            let num : i32 = rng.gen();
            let input = num.to_string();
            assert_eq!(string_to_number(&input), num);
        }        
    }
    
    #[test]
    fn my_test_1() {
      assert_eq!(string_to_number("123456789"), 123456789);
    }

    #[test]
    fn my_test_2() {
      assert_eq!(string_to_number("-123456789"), -123456789);
    }
    
    #[test]
    fn my_test_3() {
      assert_eq!(string_to_number("-987654321"), -987654321);
    }
    
    #[test]
    fn my_test_4() {
      assert_eq!(string_to_number("987654321"), 987654321);
    }
    
    #[test]
    fn my_test_5() {
      assert_eq!(string_to_number("101010"), 101010);
    }
}