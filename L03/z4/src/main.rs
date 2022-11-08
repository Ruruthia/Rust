//  b % a
fn modulo(a: u32, b: &str) -> u32 {
    b.chars().fold(0, |m,x| (m*10 + x.to_digit(10).unwrap()) % a) as u32
}

fn last_digit(str1: &str, str2: &str) -> i32 {
    if str2 == "0" {
        return 1
    }
    let mut exponent = modulo(4, str2);
    if exponent == 0 {
        exponent = 4;
    }
    let last_digit = str1.chars().last().unwrap().to_digit(10).unwrap();
    (last_digit.pow(exponent) % 10) as i32
}

#[test]
fn returns_expected() {
  assert_eq!(last_digit("4", "1"), 4);
  assert_eq!(last_digit("4", "2"), 6);
  assert_eq!(last_digit("9", "7"), 9);
  assert_eq!(last_digit("10","10000000000"), 0);
  assert_eq!(last_digit("1606938044258990275541962092341162602522202993782792835301376","2037035976334486086268445688409378161051468393665936250636140449354381299763336706183397376"), 6);
  assert_eq!(last_digit("3715290469715693021198967285016729344580685479654510946723", "68819615221552997273737174557165657483427362207517952651"), 7);
}

#[test]
fn my_test_1() {
  assert_eq!(last_digit("0", "0"), 1);
}

#[test]
fn my_test_2() {
  assert_eq!(last_digit("5", "0"), 1);
}

#[test]
fn my_test_3() {
  assert_eq!(last_digit("62494", "73842"), 6);
}

#[test]
fn my_test_4() {
  assert_eq!(last_digit("-83144", "134781"), 4);
}

#[test]
fn my_test_5() {
  assert_eq!(last_digit("-5", "1"), 5);
}