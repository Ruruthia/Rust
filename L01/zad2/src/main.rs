fn count_red_beads(n: u32) -> u32 {
    if n < 2 { 0 } else { (n-1)*2 }
}


#[test]
fn test() {
  assert_eq!(count_red_beads(0), 0);
  assert_eq!(count_red_beads(1), 0);
  assert_eq!(count_red_beads(3), 4);
  assert_eq!(count_red_beads(5), 8);
}

#[test]
fn my_test_1() {
  assert_eq!(count_red_beads(20), 38);
}

#[test]
fn my_test_2() {
  assert_eq!(count_red_beads(40), 78);
}


#[test]
fn my_test_3() {
  assert_eq!(count_red_beads(10), 18);
}

#[test]
fn my_test_4() {
  assert_eq!(count_red_beads(17), 32);
}

#[test]
fn my_test_5() {
  assert_eq!(count_red_beads(21), 40);
}