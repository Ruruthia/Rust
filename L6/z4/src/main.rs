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

// use std::cmp;

// fn longest_slide_down(pyramid: &[Vec<u16>]) -> u16 {
// //     let floors = pyramid.len();
// //     let nums = (floors + 1)*floors/2;
// //     let mut vals = vec![pyramid[0][0]];
// //     let  concated_pyramid = pyramid.concat();
// //     for (i, v) in concated_pyramid[1..].iter().enumerate(){
// //         if i > 0 {}
// //         vals.push(cmp::max(vals[((i/2) + 1) as usize], vals[((i/2)) as usize]) + v)
    
// //     }
// //     println!("{:?}", pyramid.concat());
//     let mut vals = vec![pyramid[0][0]];
//     for (i, v) in pyramid[1..].iter().enumerate(){
//         for (j, k) in v.iter().enumerate() {
//             if j == 0 {
//                 println!("start");
//                 println!("{:?}", ((2_u32.pow((i +1) as u32) + (j as u32))/2) as usize);
//                 vals.push(vals[((2_u32.pow((i ) as u32)  + (j as u32))/2) as usize]);
//             }
//             else if j == (i + 1)  {
//                 println!("end");
//                 println!("{:?}", ((2_u32.pow((i +1) as u32) -1 + (j as u32))/2) as usize);
//                 vals.push(vals[((2_u32.pow((i + 1) as u32) -1 + (j as u32))/2) as usize]);
//             }
//             else {
//                 println!("mid");
//                 println!("{:?}", ((2_u32.pow((i+1) as u32) -1 + (j as u32))/2) as usize);
//                 vals.push(cmp::max(vals[((2_u32.pow((i) as u32) -1 + (j as u32))/2) as usize], vals[((2_u32.pow((i) as u32) + (j as u32))/2) as usize]) + k)
//             }
//         }
// //         
//     }
//     println!("{:?}",vals);
//     8
// }