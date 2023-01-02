fn main() {
    let mut stacks = vec![
        "bsvzgpw".to_string(),
        "jvbczf".to_string(),
        "vlmhnzdc".to_string(),
        "ldmzpfjb".to_string(),
        "vfcgjbqh".to_string(),
        "gfqtslb".to_string(),
        "lgczv".to_string(),
        "nlg".to_string(),
        "jfhc".to_string()];

    let mut lines = include_str!("../input.txt").lines();
    for line in lines {
        let splitted_line: Vec<usize> = line.split_whitespace().filter_map(|s| s.parse::<usize>().ok()).collect::<Vec<usize>>();
        let how_many = splitted_line[0];
        let from = splitted_line[1] - 1;
        let to = splitted_line[2] - 1;
        let mut items: String = (0..how_many).map(|_| stacks[from].pop().unwrap()).collect();
        // PART 2
        items = items.chars().rev().collect();
        stacks[to].push_str(items.as_str());
    }
    println!("{:?}", stacks);
}
