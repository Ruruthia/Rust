use std::collections::HashSet;

fn simplify_code(code: &str) -> String {
    let mut simplified_code = String::new();
    let mut times = 0;

    for c in code.chars(){
        if c.is_ascii_alphabetic() {
            if times > 0 {
                for _ in 0..(times - 1) {
                    simplified_code.push(simplified_code.chars().last().unwrap());
                }  
            }
            times = 0;
            simplified_code.push(c);
        }
        else {
            times = times * 10 + c.to_digit(10).unwrap();
        }
    }
    if times > 0 {
            for _ in 0..(times - 1) {
                simplified_code.push(simplified_code.chars().last().unwrap());
        }  
    }
    simplified_code  
}


pub fn execute(code: &str) -> String {
    let mut position = (0, 0);
    let mut dir = 'R';
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert(position);
    let simplified_code = simplify_code(code);
    for c in simplified_code.chars() {
        if c == 'F' {
            match dir {
                'R' => position = (position.0, position.1 + 1),
                'L' => position = (position.0, position.1 - 1),
                'U' => position = (position.0 - 1, position.1),
                'D' => position = (position.0 + 1, position.1),
                _ => (),
            }
            visited.insert(position);
        }
        else if c == 'R' {
            match dir {
                'R' => dir = 'D',
                'L' => dir = 'U',
                'U' => dir = 'R',
                'D' => dir = 'L',
                _ => (),
            }
        }
        else if c == 'L' {
            match dir {
                'R' => dir = 'U',
                'L' => dir = 'D',
                'U' => dir = 'L',
                'D' => dir = 'R',
                _ => (),
            }
        } 
    }
    let min_x = visited.iter().min_by_key(|pos| pos.0).unwrap().0;
    let max_x = visited.iter().max_by_key(|pos| pos.0).unwrap().0;
    let min_y = visited.iter().min_by_key(|pos| pos.1).unwrap().1;
    let max_y = visited.iter().max_by_key(|pos| pos.1).unwrap().1;
    
    (min_x..=max_x)
    .map(|x| {
        (min_y..=max_y)
            .map(|y| if visited.contains(&(x, y)) { '*' } else { ' ' })
            .collect::<String>()
    })
    .collect::<Vec<_>>()
    .join("\r\n")
}


#[cfg(test)]
macro_rules! expect_equal {
  ($actual:expr, $expected:expr $(,)*) => {{
    let actual = $actual;
    let expected = $expected;
    assert_eq!(actual, expected, "\ngot:\n{}\n\nexpected:\n{}\n", actual, expected);
  }};
}

#[cfg(test)]
mod tests {
use super::execute;

    #[test]
    fn examples_in_description() {
        expect_equal!(execute(""), "*");
        expect_equal!(execute("FFFFF"), "******");
        expect_equal!(
            execute("FFFFFLFFFFFLFFFFFLFFFFFL"),
            "******\r\n*    *\r\n*    *\r\n*    *\r\n*    *\r\n******",
        );
        expect_equal!(
            execute("LFFFFFRFFFRFFFRFFFFFFF"),
            "    ****\r\n    *  *\r\n    *  *\r\n********\r\n    *   \r\n    *   ",
        );
        expect_equal!(
            execute("LF5RF3RF3RF7"),
            "    ****\r\n    *  *\r\n    *  *\r\n********\r\n    *   \r\n    *   ",
        );
    }

    #[test]
    fn my_test_1() {
        expect_equal!(execute("FLFLFLFLFLFL3F5RFRFRFRFRF"), "*******\r\n**   **");
    }

    #[test]
    fn my_test_2() {
        expect_equal!(execute("FLFRFLFRFLFR3F5FLFRFLFRFLFR"), "  *******\r\n **    **\r\n**    ** \r\n*    **  ");
    }

    #[test]
    fn my_test_3() {
        expect_equal!(execute("FL3F5R2F4R2F5L3F5RF2RF2RF2RF2RF"), "    **\r\n     *\r\n     *\r\n     *\r\n***  *\r\n* *  *\r\n******");
    }

    #[test]
    fn my_test_4() {
        expect_equal!(execute("L3F5L3F5FFRLRFFFFL2F4RF6"), "             *\r\n      *      *\r\n      *      *\r\n      *      *\r\n      *      *\r\n**************");
    }

    #[test]
    fn my_test_5() {
        expect_equal!(execute("FRF2RF3RF4RF5RF6RF7RF8RF9RF10"), "**********\r\n*        *\r\n* ****** *\r\n* *    * *\r\n* * ** * *\r\n* *  * * *\r\n* **** * *\r\n*      * *\r\n******** *\r\n         *\r\n         *");
    }

}