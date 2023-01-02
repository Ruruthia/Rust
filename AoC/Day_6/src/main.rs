// fn main() {
//     let mut signals = include_str!("../input.txt");
    
//     for (i, c) in signals.chars().enumerate() {
//         let mut seen = std::collections::HashSet::new();
//         seen.insert(c);
//         seen.insert(signals[(i + 1)..].chars().nth(0).unwrap());
//         seen.insert(signals[(i + 1)..].chars().nth(1).unwrap());
//         seen.insert(signals[(i + 1)..].chars().nth(2).unwrap());
//         if seen.len() == 4 {
//             println!("{:?}", i+4);
//             break;
//         }
//     }
// }


fn main() {
    let mut signals = include_str!("../input.txt");
    
    for (i, c) in signals.chars().enumerate() {
        let mut seen = std::collections::HashSet::new();
        seen.insert(c);
        for j in 0..13 {
            seen.insert(signals[(i + 1)..].chars().nth(j).unwrap());
        }
        if seen.len() == 14 {
            println!("{:?}", i+14);
            break;
        }
    }
 
}