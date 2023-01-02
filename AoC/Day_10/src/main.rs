// fn update_result(time: i32, x: i32) -> i32 {
    
//     if (time + 20) % 40 == 0 {
//         return time * x;
        
//     }
//     return 0;
// }

// fn main() {
//     let mut time = 0;
//     let mut x = 1;
//     let mut result = 0;
//     let input = include_str!("input.txt");
//     for line in input.lines() {
//         time += 1;
//         result += update_result(time, x);
//         if line.starts_with("addx") {
//             time += 1;
//             result += update_result(time, x);
//             let value = line.split("addx ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
//             x += value;  
//         }
//     }
//     println!("{:?}", result);
// }

use std::collections::HashMap;

fn update_image(time: i32, x: i32, image: &mut HashMap<i32, char>) {
    let pos = time % 40;
    if ((x-1)..(x+2)).contains(&pos) {
        image.insert(time, '#');
    } else {
        image.insert(time, '.');
    }
}

fn draw(image: HashMap<i32, char>) {
    for i in 0..6 {
        for j in 0..40 {
            print!("{:?}", image[&(i*40 + j)]);
        }
        println!("");
    }
}

fn main() {
    let mut time = 0;
    let mut x = 1;
    let mut result = 0;
    let mut image: HashMap<i32, char> = HashMap::new();
    let input = include_str!("input.txt");
    for line in input.lines() {
        update_image(time, x, &mut image);
        time += 1;
        if line.starts_with("addx") {
            update_image(time, x, &mut image);
            time += 1;
            let value = line.split("addx ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
            x += value;  
        }
    }
    draw(image);
}