use std::collections::HashSet;

fn check_row(row: Vec<u32>, n: usize) -> bool {
    if row.len() != n {
        return false;
    }
    let mut elements = HashSet::new();
    for el in row {
        if el < 1 || (el as usize) > n {
            return false;
        }
        elements.insert(el);
    }
    if elements.len() != n {
        return false;
    }
    true
}

struct Sudoku{
    data: Vec<Vec<u32>>,
}


impl Sudoku{
    fn is_valid(&self) -> bool {
      let n = self.data.len();
      for row in &self.data {
        if !check_row(row.to_vec(), n){
            return false;
        }
      }
      for i in 0..n {
        let mut col = vec![];
        for j in 0..n{
            col.push(self.data[j][i])
        }
        if !check_row(col, n){
            return false;
        }
      }
      let size = (n as f32).sqrt() as usize;
      let mut squares = vec![vec![]; n];
      for i in 0..n{
        for j in 0..n{
            let square = (i/size) * size + j/size;
            println!("{}", square);
            squares[square].push(self.data[i][j])
        }
      }
      for square in squares{
        if !check_row(square, n){
            return false;
        }
      }  
    true
    }
}

#[test]
fn good_sudoku() {
    let good_sudoku_1 = Sudoku{
        data: vec![
                vec![7,8,4, 1,5,9, 3,2,6],
                vec![5,3,9, 6,7,2, 8,4,1],
                vec![6,1,2, 4,3,8, 7,5,9],

                vec![9,2,8, 7,1,5, 4,6,3],
                vec![3,5,7, 8,4,6, 1,9,2],
                vec![4,6,1, 9,2,3, 5,8,7],
                
                vec![8,7,6, 3,9,4, 2,1,5],
                vec![2,4,3, 5,6,1, 9,7,8],
                vec![1,9,5, 2,8,7, 6,3,4]
            ]
    };
    
    let good_sudoku_2 = Sudoku{
        data: vec![
                vec![1, 4,  2, 3],
                vec![3, 2,  4, 1],
        
                vec![4, 1,  3, 2],
                vec![2, 3,  1, 4],
            ]
    };
    assert!(good_sudoku_1.is_valid());
    assert!(good_sudoku_2.is_valid());
}

#[test]
fn bad_sudoku() {
    let bad_sudoku_1 = Sudoku{
        data: vec![
                vec![1,2,3, 4,5,6, 7,8,9],
                vec![1,2,3, 4,5,6, 7,8,9],
                vec![1,2,3, 4,5,6, 7,8,9],

                vec![1,2,3, 4,5,6, 7,8,9],
                vec![1,2,3, 4,5,6, 7,8,9],
                vec![1,2,3, 4,5,6, 7,8,9],
                
                vec![1,2,3, 4,5,6, 7,8,9],
                vec![1,2,3, 4,5,6, 7,8,9],
                vec![1,2,3, 4,5,6, 7,8,9],
            ]
    };
    
    let bad_sudoku_2 = Sudoku{
        data: vec![
                vec![1,2,3,4,5],
                vec![1,2,3,4],
                vec![1,2,3,4],
                vec![1],
            ]
    };
    assert!(!bad_sudoku_1.is_valid());
    assert!(!bad_sudoku_2.is_valid());
}

// check shape
// check if numbers are in good range
// check sums