//  TODO: Refactor

struct Memo {
    john_cache: Vec<i32>,
    ann_cache: Vec<i32>
}

impl Memo {
    fn new(n: i32) -> Memo {
        let john_cache = vec![-1; n as usize];
        let ann_cache = vec![-1; n as usize];
        Memo { john_cache, ann_cache}
    }

    fn f(&mut self, arg: usize, person: &str) -> i32 {
        if person == "john" {
            match self.john_cache[arg] {
                -1 => {
                    let result = match arg {
                        0 => 0,
                        n => {
                            let temp = self.f(n-1, "john") as usize;
                            n - self.f(temp, "ann") as usize
                        },
                    };
                    self.john_cache[arg] = result as i32;
                    result as i32
                }
                n => n
            }
        }
        else {
            match self.ann_cache[arg] {
                -1 => {
                    let result = match arg {
                        0 => 1,
                        n => {
                            let temp = self.f(n-1, "ann") as usize;
                            n - self.f(temp, "john") as usize
                        },
                       
                    };
                    self.ann_cache[arg] = result as i32;
                    result as i32
                }
                n => n
            }       
        }
    }
}




fn john(n: i32) -> Vec<i32> {
    let mut memo = Memo::new(n);
    for i in 0..n {
        _ = memo.f(i as usize, "john");
    }
    memo.john_cache
}

fn ann(n: i32) -> Vec<i32> {
    let mut memo = Memo::new(n);
    for i in 0..n {
        _ = memo.f(i as usize, "ann");
    }
    memo.ann_cache
}

fn sum_john(n: i32) -> i32 {
    let john_values = john(n);
    john_values.iter().sum()
}

fn sum_ann(n: i32) -> i32 {
    let ann_values = ann(n);
    ann_values.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_john() {
        assert_eq!(john(11), vec![0, 0, 1, 2, 2, 3, 4, 4, 5, 6, 6]);
        assert_eq!(john(14), vec![0, 0, 1, 2, 2, 3, 4, 4, 5, 6, 6, 7, 7, 8]);
    }
    #[test]
    fn test_ann() {
        assert_eq!(ann(6), vec![1, 1, 2, 2, 3, 3]);
        assert_eq!(ann(15), vec![1, 1, 2, 2, 3, 3, 4, 5, 5, 6, 6, 7, 8, 8, 9]);
    }
    #[test]
    fn test_sum_john() {
        assert_eq!(sum_john(75), 1720);
        assert_eq!(sum_john(78), 1861);
    }
    #[test]
    fn test_sum_ann() {
        assert_eq!(sum_ann(115), 4070);
        assert_eq!(sum_ann(150), 6930);
    }
}