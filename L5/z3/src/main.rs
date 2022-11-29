fn find_digit(num: i32, nth: i32) -> i32 {
    if nth <= 0 {
        return -1;
    }
    let mut abs_res = num.abs();
    for _ in 0..nth-1 {
          abs_res = abs_res / 10;
    }
    abs_res % 10
  }

#[test]
fn example_test() {
  assert_eq!(find_digit(5673, 4), 5);
  assert_eq!(find_digit(129, 2), 2);
  assert_eq!(find_digit(-2825, 3), 8);
  assert_eq!(find_digit(-456, 4), 0);
  assert_eq!(find_digit(0, 20), 0);
  assert_eq!(find_digit(65, 0), -1);
  assert_eq!(find_digit(24, -8), -1);
}

#[test]
fn my_test_1() {
  assert_eq!(find_digit(0, 0), -1);
}

#[test]
fn my_test_2() {
  assert_eq!(find_digit(0, 10), 0);
}

#[test]
fn my_test_3() {
  assert_eq!(find_digit(-21294383, 7), 1);
}

#[test]
fn my_test_4() {
  assert_eq!(find_digit(002631, 4), 2);
}

#[test]
fn my_test_5() {
  assert_eq!(find_digit(89361213, 100), 0);
}