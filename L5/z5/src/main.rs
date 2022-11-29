use std::collections::HashSet;

fn sum_pairs(ints: &[i8], s: i8) -> Option<(i8, i8)> {
    let mut values = HashSet::new();
    for x in ints {
        if values.contains(&(s-*x)) {
            return Some((s-*x, *x));
        } else {
            values.insert(*x);
        }
    }
    None
}

#[test]
fn returns_expected() {
    let l1 = [1, 4, 8, 7, 3, 15];
    let l2 = [1, -2, 3, 0, -6, 1];
    let l3 = [20, -13, 40];
    let l4 = [1, 2, 3, 4, 1, 0];
    let l5 = [10, 5, 2, 3, 7, 5];
    let l6 = [4, -2, 3, 3, 4];
    let l7 = [0, 2, 0];
    let l8 = [5, 9, 13, -3];
    assert_eq!(sum_pairs(&l1, 8), Some((1, 7)));
    assert_eq!(sum_pairs(&l2, -6), Some((0, -6)));
    assert_eq!(sum_pairs(&l3, -7), None);
    assert_eq!(sum_pairs(&l4, 2), Some((1, 1)));
    assert_eq!(sum_pairs(&l5, 10), Some((3, 7)));
    assert_eq!(sum_pairs(&l6, 8), Some((4, 4)));
    assert_eq!(sum_pairs(&l7, 0), Some((0, 0)));
    assert_eq!(sum_pairs(&l8, 10), Some((13, -3)));
}

#[test]
fn my_test_1() {
    assert_eq!(sum_pairs(&[], 10), None)
}

#[test]
fn my_test_2() {
    assert_eq!(sum_pairs(&[13, 2, 1, 4, 21, -12, 1, 3, 22], 10), Some((-12, 22)))
}

#[test]
fn my_test_3() {
    assert_eq!(sum_pairs(&[13, 2, 1, 4, 21, -12, 1, 3, 22], 15), Some((13, 2)))
}

#[test]
fn my_test_4() {
    assert_eq!(sum_pairs(&[0, 1, 2, 3, 4, -1, -2, -3], 0), Some((1, -1)))
}

#[test]
fn my_test_5() {
    assert_eq!(sum_pairs(&[0, 1, 2, 3, 4, -1, -2, -3], 6), Some((2, 4)))
}