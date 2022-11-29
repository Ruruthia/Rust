use std::cmp;
fn longest_slide_down(pyramid: &[Vec<u16>]) -> u16 {
    let mut merged_pyramid = pyramid.to_vec();

    while merged_pyramid.len() > 1 {

        let last_row = merged_pyramid.pop().unwrap();
        let second_to_last_row = merged_pyramid.pop().unwrap();

        let mut merged_row: Vec<u16> = Vec::new();

        for (i, value) in second_to_last_row.iter().enumerate() {
            merged_row.push(cmp::max(last_row[i], last_row[i+1]) + value);
        };

        merged_pyramid.push(merged_row);
    }
    
    merged_pyramid[0][0]
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small() {
        let small = vec![
            vec![3],
            vec![7, 4],
            vec![2, 4, 6],
            vec![8, 5, 9, 3]
        ];
        assert_eq!(longest_slide_down(&small), 23, "It should work for small pyramids");
    }
    
    #[test]
    fn test_medium() {
        let medium = vec![
  vec![75],
  vec![95, 64],
  vec![17, 47, 82],
  vec![18, 35, 87, 10],
  vec![20,  4, 82, 47, 65],
  vec![19,  1, 23, 75,  3, 34],
  vec![88,  2, 77, 73,  7, 63, 67],
  vec![99, 65,  4, 28,  6, 16, 70, 92],
  vec![41, 41, 26, 56, 83, 40, 80, 70, 33],
  vec![41, 48, 72, 33, 47, 32, 37, 16, 94, 29],
  vec![53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14],
  vec![70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57],
  vec![91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48],
  vec![63, 66,  4, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31],
  vec![ 4, 62, 98, 27, 23,  9, 70, 98, 73, 93, 38, 53, 60,  4, 23]
  ];
        assert_eq!(longest_slide_down(&medium), 1074, "It should work for medium pyramids");
    }

    #[test]
    fn my_test_1() {
        let pyramid = vec![
            vec![0],
        ];
        assert_eq!(longest_slide_down(&pyramid), 0);
    }


    #[test]
    fn my_test_2() {
        let pyramid = vec![
            vec![6],
            vec![18, 10],
            vec![21, 5, 13],
            vec![0, 5, 9, 13]
        ];
        assert_eq!(longest_slide_down(&pyramid), 50);
    }


    #[test]
    fn my_test_3() {
        let pyramid = vec![
            vec![1],
            vec![1, 2],
            vec![1, 2, 3],
            vec![1, 2, 3, 4],
            vec![1, 2, 3, 4, 5],
            vec![1, 2, 3, 4, 5, 6],
            vec![1, 2, 3, 4, 5, 6, 7],
        ];
        assert_eq!(longest_slide_down(&pyramid), 28);
    }


    #[test]
    fn my_test_4() {
        let pyramid = vec![
            vec![5],
            vec![8, 5],
            vec![11, 20, 4],
            vec![82, 100, 123, 97],
            vec![0, 0, 0, 0, 0],
        ];
        assert_eq!(longest_slide_down(&pyramid), 156);
    }


    #[test]
    fn my_test_5() {
        let pyramid = vec![
            vec![0],
            vec![0, 0],
            vec![200, 100, 400],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![10, 20, 30, 40, 50, 60, 500, 800, 80],
        ];
        assert_eq!(longest_slide_down(&pyramid), 1200);
    }
}