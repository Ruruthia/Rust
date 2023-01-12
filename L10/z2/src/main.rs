fn dig_pow(n: i64, p: i32) -> i64 {
    let mut power = p as u32;
    let mut sum : i64 = 0;

    for c in n.to_string().chars(){
        let digit = c.to_digit(10).unwrap() as i64;
        sum += digit.pow(power) as i64;
        power += 1;
    }

    match sum % n {
        0 => sum / n,
        _ => -1
    }
}

#[cfg(test)]
    mod tests {
    use super::*;

    fn dotest(n: i64, p: i32, exp: i64) -> () {
        let ans = dig_pow(n, p);
        assert_eq!(ans, exp);
    }
    
    #[test]
    fn basic_tests() {
        dotest(89, 1, 1);
        dotest(92, 1, -1);
        dotest(46288, 3, 51);
    }
}
