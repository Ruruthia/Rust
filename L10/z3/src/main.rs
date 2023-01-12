use std::collections::HashMap;

fn comp(a: Vec<i64>, b: Vec<i64>) -> bool {
    let mut a_squared_map: HashMap<i64, usize> = HashMap::new();
    let mut b_map: HashMap<i64, usize> = HashMap::new();
    for el in a {
        *a_squared_map.entry(el.pow(2)).or_default() += 1;
    }
    for el in b{
        *b_map.entry(el).or_default() += 1;
    }
    a_squared_map == b_map
}

fn testing(a: Vec<i64>, b: Vec<i64>, exp: bool) -> () {
    assert_eq!(comp(a, b), exp)
}

#[test]
fn tests_comp() {

    let a1 = vec![121, 144, 19, 161, 19, 144, 19, 11];
    let a2 = vec![11*11, 121*121, 144*144, 19*19, 161*161, 19*19, 144*144, 19*19];
    testing(a1, a2, true);
    let a1 = vec![121, 144, 19, 161, 19, 144, 19, 11];
    let a2 = vec![11*21, 121*121, 144*144, 19*19, 161*161, 19*19, 144*144, 19*19];
    testing(a1, a2, false);

}