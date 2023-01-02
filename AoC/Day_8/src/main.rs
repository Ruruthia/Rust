// fn is_visible(input_vec: Vec<Vec<u32>>, x: usize, y: usize) -> bool {
//     if x == 0 || y == 0 || x == (input_vec.len() - 1) || y == (input_vec[x].len() - 1) {
//         return true
//     }
//     let left_side_max = input_vec[x][..y].iter().max().unwrap();
//     let right_side_max = input_vec[x][y+1..].iter().max().unwrap();
//     let up_side_max = input_vec[..x].iter().map(|row| row[y]).max().unwrap();
//     let down_side_max = input_vec[x+1..].iter().map(|row| row[y]).max().unwrap();
//     let maxes = vec![*left_side_max, *right_side_max, up_side_max, down_side_max];
//     let min_max = maxes.iter().min().unwrap();
//     if min_max < &input_vec[x][y] {
//         return true
//     }
//     return false
// }


// fn main() {
//     let input = include_str!("input.txt");
//     let input_vec : Vec<Vec<u32>> = input.lines().map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect()).collect();
//     let mut visible = 0;
//     for (x, row) in input_vec.clone().iter().enumerate() {
//         for (y, _) in row.iter().enumerate() {
//             if is_visible(input_vec.clone(), x , y) {
//                 visible += 1;
//             }
//         }
//     }
//     println!("{:?}", visible);
// }


fn scenic_score(input_vec: Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    if x == 0 || y == 0 || x == (input_vec.len() - 1) || y == (input_vec[x].len() - 1) {
        return 0
    }
    let mut score = 1;
    let mut visible = 0;
    for h in input_vec[x][..y].iter().rev() {
        visible += 1;
        if h >= &input_vec[x][y] {
            break;
        }
        
    }
    score *= visible;
    visible = 0;
    for h in input_vec[x][y+1..].iter() {
        visible += 1;
        if h >= &input_vec[x][y] {
            break;
        }
        
    }
    score *= visible;
    visible = 0;
    for h in input_vec[..x].iter().map(|row| row[y]).rev() {
        visible += 1;
        if h >= input_vec[x][y] {
            break;
        }
        
    }
    score *= visible;
    visible = 0;
    for h in input_vec[x+1..].iter().map(|row| row[y]) {
        visible += 1;
        if h >= input_vec[x][y] {
            break;
        }
        
    }
    score *= visible;
    return score;
}

fn main() {
    let input = include_str!("input.txt");
    let input_vec : Vec<Vec<u32>> = input.lines().map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect()).collect();
    let mut max_score = 0;
    for (x, row) in input_vec.clone().iter().enumerate() {
        for (y, _) in row.iter().enumerate() {
            if scenic_score(input_vec.clone(), x , y) > max_score {
                max_score = scenic_score(input_vec.clone(), x , y);
            }
        }
    }
    println!("{:?}", max_score);
}