fn mumble(i: usize, ch: char)->String {
    let mut s = String::new();
    s.push(ch.to_ascii_uppercase());
    for _ in 0..i {
        s.push(ch.to_ascii_lowercase());
    }
    s
}


fn accum(s:&str)->String {
    s.chars().enumerate().map(|(i, c)| mumble(i, c)).collect::<Vec<String>>().join("-")
}


#[test]
fn basic_tests() {
  assert_eq!(accum("ZpglnRxqenU"), "Z-Pp-Ggg-Llll-Nnnnn-Rrrrrr-Xxxxxxx-Qqqqqqqq-Eeeeeeeee-Nnnnnnnnnn-Uuuuuuuuuuu");
  assert_eq!(accum("NyffsGeyylB"), "N-Yy-Fff-Ffff-Sssss-Gggggg-Eeeeeee-Yyyyyyyy-Yyyyyyyyy-Llllllllll-Bbbbbbbbbbb");
  assert_eq!(accum("MjtkuBovqrU"), "M-Jj-Ttt-Kkkk-Uuuuu-Bbbbbb-Ooooooo-Vvvvvvvv-Qqqqqqqqq-Rrrrrrrrrr-Uuuuuuuuuuu");
  assert_eq!(accum("EvidjUnokmM"), "E-Vv-Iii-Dddd-Jjjjj-Uuuuuu-Nnnnnnn-Oooooooo-Kkkkkkkkk-Mmmmmmmmmm-Mmmmmmmmmmm");
  assert_eq!(accum("HbideVbxncC"), "H-Bb-Iii-Dddd-Eeeee-Vvvvvv-Bbbbbbb-Xxxxxxxx-Nnnnnnnnn-Cccccccccc-Ccccccccccc");
}

#[test]
fn my_test_1() {
    assert_eq!(accum(""), "");
}

#[test]
fn my_test_2() {
    assert_eq!(accum("ABCDEFG"), "A-Bb-Ccc-Dddd-Eeeee-Ffffff-Ggggggg");
}

#[test]
fn my_test_3() {
    assert_eq!(accum("abcdefg"), "A-Bb-Ccc-Dddd-Eeeee-Ffffff-Ggggggg");
}

#[test]
fn my_test_4() {
    assert_eq!(accum("aBcDeFg"), "A-Bb-Ccc-Dddd-Eeeee-Ffffff-Ggggggg");
}

#[test]
fn my_test_5() {
    assert_eq!(accum("AbCdEfG"), "A-Bb-Ccc-Dddd-Eeeee-Ffffff-Ggggggg");
}