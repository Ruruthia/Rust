fn print(n: i32) -> Option<String> {
    if n < 0 || n % 2 == 0 {
        return None;
    } 
    let mut diamond = String::new();
    let half = n / 2;
    for i in 0..(half+1) {
        for _ in 0..half - i {
            diamond.push(' ');
        }
        for _ in 0..2 * i + 1 {
            diamond.push('*');
        }
        diamond.push('\n');
    }
    for i in (0..half).rev() {
        for _ in 0..half - i {
            diamond.push(' ');
        }
        for _ in 0..2 * i + 1 {
            diamond.push('*');
        }
        diamond.push('\n');
    }
    Some(diamond)
}

#[test]
fn basic_test() {
    assert_eq!(print(3), Some(" *\n***\n *\n".to_string()) );
    assert_eq!(print(5), Some("  *\n ***\n*****\n ***\n  *\n".to_string()) );
    assert_eq!(print(-3), None);
    assert_eq!(print(2), None);
    assert_eq!(print(0), None);
    assert_eq!(print(1), Some("*\n".to_string()) );
  }