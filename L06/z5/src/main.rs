fn descending_order(x: u64) -> u64 {
    let mut chars_vec = x.to_string().chars().collect::<Vec<char>>();
    chars_vec.sort_by(|a, b| b.cmp(a));
    chars_vec.iter().fold(0, |acc, elem| acc * 10 + (elem.to_digit(10).unwrap() as u64))
}

#[test]
fn returns_expected() {
    assert_eq!(descending_order(0), 0);
    assert_eq!(descending_order(1), 1);
    assert_eq!(descending_order(15), 51);
    assert_eq!(descending_order(1021), 2110);
    assert_eq!(descending_order(123456789), 987654321);
    assert_eq!(descending_order(145263), 654321);
    assert_eq!(descending_order(1254859723), 9875543221);
}

#[test]
fn my_test_1() {
    assert_eq!(descending_order(961497134), 997644311);
}

#[test]
fn my_test_2() {
    assert_eq!(descending_order(0123456789), 987654321);
}

#[test]
fn my_test_3() {
    assert_eq!(descending_order(9876543210), 9876543210);
}

#[test]
fn my_test_4() {
    assert_eq!(descending_order(10203040506070), 76543210000000);
}

#[test]
fn my_test_5() {
    assert_eq!(descending_order(846204712), 876442210);
}