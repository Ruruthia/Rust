use std::cmp;
fn longest_slide_down(pyramid: &[Vec<u16>]) -> u16 {
    let mut vector = pyramid.to_vec();

    while vector.len() > 1 {

    let last = vector.pop().unwrap();
    let ante = vector.pop().unwrap();

    let mut new: Vec<u16> = Vec::new();

    for (i, value) in ante.iter().enumerate() {
        new.push(cmp::max(last[i], last[i+1]) + value);
    };

    vector.push(new);
    };
    
    vector[0][0]
    
}