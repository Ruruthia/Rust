fn square_area_to_circle(size:f64) -> f64 {
    return  std::f64::consts::PI * (size/4.0)
}

fn assert_close(a:f64, b:f64, epsilon:f64) {
  assert!( (a-b).abs() < epsilon, "Expected: {}, got: {}",b,a);
}

#[test]
fn my_test_1() {
  assert_close(square_area_to_circle(5.0), 3.92699081699, 1e-8);
}

#[test]
fn my_test_2() {
  assert_close(square_area_to_circle(2.5), 1.96349540849, 1e-8);
}

#[test]
fn my_test_3() {
  assert_close(square_area_to_circle(8.75), 6.87223392973, 1e-8);
}

#[test]
fn my_test_4() {
  assert_close(square_area_to_circle(31.0), 24.3473430653, 1e-8);
}

#[test]
fn my_test_5() {
  assert_close(square_area_to_circle(100.0), 78.5398163397, 1e-8);
}