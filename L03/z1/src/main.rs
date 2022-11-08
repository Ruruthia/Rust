fn sum_product(a: &[i32], b: &[i32]) -> i32 {
    a.iter()
        .zip(b.iter())
        .map(|(a, b)| a * b)
        .sum()
}

fn calculate_score(warriors_str: &str, worth: &[i32]) -> i32 {
    let warriors_array = warriors_str.split_whitespace().map(|d| d.parse().unwrap()).collect::<Vec<i32>>();
    sum_product(worth, &warriors_array)
}

fn good_vs_evil(good: &str, evil: &str) -> String {
  let good_score = calculate_score(good, &[1, 2, 3, 3, 4, 10]);
  let evil_score = calculate_score(evil, &[1, 2, 2, 2, 3, 5, 10]);
  if good_score > evil_score {
      String::from("Battle Result: Good triumphs over Evil")
  } else if good_score == evil_score {
      String::from("Battle Result: No victor on this battle field")
  } else {
    String::from("Battle Result: Evil eradicates all trace of Good")    
  }
}

#[test]
fn returns_expected() {
    assert_eq!(good_vs_evil("0 0 0 0 0 10", "0 0 0 0 0 0 0"), "Battle Result: Good triumphs over Evil");
    assert_eq!(good_vs_evil("0 0 0 0 0 0", "0 0 0 0 0 0 10"), "Battle Result: Evil eradicates all trace of Good");
    assert_eq!(good_vs_evil("0 0 0 0 0 10", "0 0 0 0 0 0 10"), "Battle Result: No victor on this battle field");
}

#[test]
fn my_test_1() {
    assert_eq!(good_vs_evil("1 2 3 4 5 6", "1 2 3 4 5 5 5"), "Battle Result: Evil eradicates all trace of Good");
}

#[test]
fn my_test_2() {
    assert_eq!(good_vs_evil("0 0 0 0 0 0", "0 0 0 0 0 0 0"), "Battle Result: No victor on this battle field");
}

#[test]
fn my_test_3() {
    assert_eq!(good_vs_evil("6 7 2 8 5 1", "3 5 11 2 2 2 0"), "Battle Result: Good triumphs over Evil");
}

#[test]
fn my_test_4() {
    assert_eq!(good_vs_evil("100 100 100 100 100 100", "100 100 100 100 100 100 100"), "Battle Result: Evil eradicates all trace of Good");
}

#[test]
fn my_test_5() {
    assert_eq!(good_vs_evil("1 1 1 1 1 1", "1 1 0 1 1 1 1"), "Battle Result: No victor on this battle field");
}
