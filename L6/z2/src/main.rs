mod solution {

    pub fn range_extraction(a: &[i32]) -> String {
        let mut start = a[0];
        let mut current = start;
        let mut next = 0;
        let mut text = "".to_string();
        for i in 1..a.len(){
            next = a[i];
            if next != current + 1{
                if current - start >= 2 {
                    text = format!("{},{}-{}", text, start.to_string(), current.to_string());
                    println!("{:?}", vec![start, current]);
                }
                else if current - start == 1{
                    text = format!("{},{},{}", text, start.to_string(), current.to_string());
                    println!("{:?}", (start, current));
                }
                else {
                    text = format!("{},{}", text, start.to_string());
                    println!("{:?}", start);
                }
                start = next;
            }
            current = next;
        }
        if current - start >= 2 {
            text = format!("{},{}-{}", text, start.to_string(), current.to_string());
            println!("{:?}", vec![start, current]);
        }
        else if current - start == 1{
            text = format!("{},{},{}", text, start.to_string(), current.to_string());
            println!("{:?}", (start, current));
        }
        else {
            text = format!("{},{}", text, start.to_string());
            println!("{:?}", start);
        }
        start = next;

    text[1..].to_string()
    }
}