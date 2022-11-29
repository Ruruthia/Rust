mod solution {

    pub fn range_extraction(a: &[i32]) -> String {
        let mut start = a[0];
        let mut current = start;
        let mut next = 0;
        let mut text = "".to_string();
        for i in 1..(a.len()+1){
            if i < a.len() {
                next = a[i];
            }
            if next != current + 1{
                if current - start >= 2 {
                    text = format!("{},{}-{}", text, start.to_string(), current.to_string());               
                }
                else if current - start == 1{
                    text = format!("{},{},{}", text, start.to_string(), current.to_string());     
                }
                else {
                    text = format!("{},{}", text, start.to_string());
                }
                start = next;
            }
            current = next;
        }
        text[1..].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(solution::range_extraction(&[-6,-3,-2,-1,0,1,3,4,5,7,8,9,10,11,14,15,17,18,19,20]), "-6,-3-1,3-5,7-11,14,15,17-20");	
        assert_eq!(solution::range_extraction(&[-3,-2,-1,2,10,15,16,18,19,20]), "-3--1,2,10,15,16,18-20");
    }

    #[test]
    fn my_test_1() {
        assert_eq!(solution::range_extraction(&[1,2,3,4,5,6,7,8,9,10,12,13,14,17,18]), "1-10,12-14,17,18");	
    }

    #[test]
    fn my_test_2() {
        assert_eq!(solution::range_extraction(&[-2,-1,0,11,15,20,21,22]), "-2-0,11,15,20-22");	
    }

    #[test]
    fn my_test_3() {
        assert_eq!(solution::range_extraction(&[-100,-99,0,13,15,100,101,102,400,472,473,474,482,483]), "-100,-99,0,13,15,100-102,400,472-474,482,483");	
    }

    #[test]
    fn my_test_4() {
        assert_eq!(solution::range_extraction(&[0,1,2,3,4,5,6,7,8,9,10,11]), "0-11");	
    }

    #[test]
    fn my_test_5() {
        assert_eq!(solution::range_extraction(&[-10, -8, -7, -6, 0, 6, 7, 8, 10]), "-10,-8--6,0,6-8,10");	
    }
}
