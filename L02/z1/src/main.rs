fn camel_case(str: &str) -> String {
    str.split_whitespace().map(|s| s[0..1].to_uppercase() + &s[1..]).collect::<String>()
}

#[test]
fn sample_test() {
  assert_eq!(camel_case("test case"), "TestCase");
  assert_eq!(camel_case("camel case method"), "CamelCaseMethod");
  assert_eq!(camel_case("say hello "), "SayHello");
  assert_eq!(camel_case(" camel case word"), "CamelCaseWord");
  assert_eq!(camel_case(""), "");
}

#[test]
fn my_test_1() {
    assert_eq!(camel_case("maria wyrzykowska"), "MariaWyrzykowska");
}

#[test]
fn my_test_2() {
    assert_eq!(camel_case("test nr dwa"), "TestNrDwa");
}

#[test]
fn my_test_3() {
    assert_eq!(camel_case("test nr 2"), "TestNr2");
}

#[test]
fn my_test_4() {
    assert_eq!(camel_case("test  z  spacjami"), "TestZSpacjami");
}

#[test]
fn my_test_5() {
    assert_eq!(camel_case("ostatni_test"), "Ostatni_test");
}