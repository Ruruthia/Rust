fn get_count(string: &str) -> usize {
    string.chars().filter(|c| "aeiou".contains(*c)).count()
}

#[test]
fn my_tests() {
  assert_eq!(get_count("abracadabra"), 5);
}

#[test]
fn my_test_1() {
  assert_eq!(get_count(""), 0);
}

#[test]
fn my_test_2() {
  assert_eq!(get_count("aeiouy"), 5);
}

#[test]
fn my_test_3() {
  assert_eq!(get_count("aaeeiioouuyy"), 10);
}

#[test]
fn my_test_4() {
  assert_eq!(get_count("ąężźć"), 0);
}

#[test]
fn my_test_5() {
  assert_eq!(get_count("12345"), 0);
}