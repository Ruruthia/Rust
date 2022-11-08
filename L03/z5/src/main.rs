fn number(bus_stops:&[(i32,i32)]) -> i32 {
    bus_stops.iter().fold(0, |a: i32, x: &(i32, i32)| (a + (x.0 - x.1)))
}

#[test]
fn returns_expected() {
  assert_eq!(number(&[(10,0),(3,5),(5,8)]), 5);
  assert_eq!(number(&[(3,0),(9,1),(4,10),(12,2),(6,1),(7,10)]), 17);
  assert_eq!(number(&[(3,0),(9,1),(4,8),(12,2),(6,1),(7,8)]), 21);
}

#[test]
fn my_test_1() {
  assert_eq!(number(&[(0,0),(0,0),(0,0)]), 0);
}

#[test]
fn my_test_2() {
  assert_eq!(number(&[(10,0),(10,0),(10,0)]), 30);
}

#[test]
fn my_test_3() {
  assert_eq!(number(&[(10,0),(10,0),(10,30)]), 0);
}

#[test]
fn my_test_4() {
  assert_eq!(number(&[(12414,1221),(7324,632),(123,500),(1230,11000),(13,50)]), 7701);
}

#[test]
fn my_test_5() {
  assert_eq!(number(&[(1234,123),(12345,1234),(1,500),(1233410,18420),(2413,410)]), 1228716);
}
