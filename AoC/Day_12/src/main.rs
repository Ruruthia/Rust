// fn main() {
//     let input = include_str!("../input.txt");
//     let grid : Vec<Vec<u8>> = input.lines().map(|l| l.as_bytes().to_vec()).collect();
//     let mut sx = 0;
//     let mut sy = 0;

//     for x in 0..grid.len() {
//         for y in 0..grid[x].len() {
//             if grid[x][y] == b'S' {
//                 (sx, sy) = (x, y);
//             }
//         } 
//     }
//     let target = b'z'; 
//     let mut visited = std::collections::HashSet::new();
//     visited.insert((sx, sy));
//     let mut queue = std::collections::VecDeque::new();
//     queue.push_back((sx, sy, b'a', 1));
//     while !queue.is_empty() {
//         // println!("{:?}", queue);
//         (x, y, h, s) = queue.pop_front()
//         if target == h {
//             println!("{:?}", s);
//             break;
//         }
//         for (dx, dy) in vec![(-1, 0), (0, -1), (1, 0), (0, 1)] {

//             let new_x = x as isize + dx;
//             let new_y = y as isize + dy;
//             if new_x < 0 || new_y < 0 {
//                 continue;
//             }

//             let (new_x, new_y) = (new_x as usize, new_y as usize);
//             if new_x >= grid.len() || new_y >= grid[0].len() {
//                 continue;
//             }

//             if visited.contains(&(new_x, new_y)) {
//                 continue;
//             }

//             let mut new_h = grid[new_x][new_y];

//             if new_h == b'E' {
//                 new_h = b'z';
//             }

//             if new_h > h && (new_h - h) > 1 {
//                 continue;
//             }
//             queue.push_back((new_x, new_y, new_h, s + 1));
//             visited.insert((new_x, new_y));
//         }
//     }
// }

fn main() {
    let mut min_path = 1000;
    let input = include_str!("../input.txt");
    let grid : Vec<Vec<u8>> = input.lines().map(|l| l.as_bytes().to_vec()).collect();
    let (mut cx, mut cy, mut h, mut s) = (0,0,0,0);
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            if grid[x][y] == b'a' {
                let (sx, sy) = (x, y);
                let target = b'z'; 
                let mut visited = std::collections::HashSet::new();
                visited.insert((sx, sy));
                let mut queue = std::collections::VecDeque::new();
                queue.push_back((sx, sy, b'a', 1));
                while !queue.is_empty() {
                    // println!("{:?}", queue);
                    if let Some((cx, cy, h, s)) = queue.pop_front() {
                    if target == h {
                        if s < min_path {
                            min_path = s;
                        }
                        break;
                    }
                    for (dx, dy) in vec![(-1, 0), (0, -1), (1, 0), (0, 1)] {
            
                        let new_x = cx as isize + dx;
                        let new_y = cy as isize + dy;
                        if new_x < 0 || new_y < 0 {
                            continue;
                        }
            
                        let (new_x, new_y) = (new_x as usize, new_y as usize);
                        if new_x >= grid.len() || new_y >= grid[0].len() {
                            continue;
                        }
            
                        if visited.contains(&(new_x, new_y)) {
                            continue;
                        }
            
                        let mut new_h = grid[new_x][new_y];
            
                        if new_h == b'E' {
                            new_h = b'z';
                        }
                        if new_h == b'S' {
                            new_h = b'a';
                        }
            
                        if new_h > h && (new_h - h) > 1 {
                            continue;
                        }
                        queue.push_back((new_x, new_y, new_h, s + 1));
                        visited.insert((new_x, new_y));
                    }
                }
            }
        
        }
    }
    } 
    println!("{}", min_path);
}

