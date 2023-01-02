use std::collections::HashMap;

fn get_filesystem_size() -> HashMap<Vec<&'static str>, i32> {
    let input = include_str!("../input.txt");

    let mut fs = HashMap::new();
    fs.insert(vec!["/"], 0);
    let mut path = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts.as_slice() {
            ["$", "ls"] => {}
            ["$", "cd", ".."] => {
                path.pop();
            }
            ["$", "cd", directory] => {
                path.push(*directory);
            }
            ["dir", directory] => {
                let mut temp_path = path.clone();
                temp_path.push(directory);
                fs.insert(temp_path, 0);
            }
            [size, _] => {
                for i in 1..=path.len() {
                    *fs.get_mut(&path[..i]).unwrap() += size.parse::<i32>().unwrap();
                }
            }
            _ => {}
        }
    }

    fs
}

fn main() {
    let filesystem = get_filesystem_size();
    // Part 1
    // println!("{}", filesystem.into_iter().filter(|x| x.1 <= 100000).map(|(_, y)| y).sum::<i32>());
    // 
    let fs_size : i32 = 70000000;
    let space_needed : i32 = 30000000;
    let unused : i32 = fs_size - filesystem.clone().get(&vec!["/"]).unwrap();
    let size_to_del : i32 = space_needed - unused;
    println!("{:?}", filesystem.into_iter().filter(|x| x.1 >= size_to_del).min_by_key(|(_, y)| *y));
}
