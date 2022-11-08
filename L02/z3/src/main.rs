fn dont_give_me_five(start: isize, end: isize) -> isize {
    
    let numbers = start..=end;
    numbers.filter(|d| !d.to_string().contains("5")).count() as isize
}

#[test]
fn returns_expected() {
    assert_eq!(dont_give_me_five(1, 9), 8);
    assert_eq!(dont_give_me_five(4, 17), 12);
}

#[test]
fn my_test_1() {
    assert_eq!(dont_give_me_five(5, 5), 0);
}

#[test]
fn my_test_2() {
    assert_eq!(dont_give_me_five(6, 6), 1);
}

#[test]
fn my_test_3() {
    assert_eq!(dont_give_me_five(5, 100), 77);
}

#[test]
fn my_test_4() {
    assert_eq!(dont_give_me_five(-5, 5), 9);
}

#[test]
fn my_test_5() {
    assert_eq!(dont_give_me_five(6, -5), 0);
}