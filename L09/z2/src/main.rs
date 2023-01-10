use std::collections::BTreeMap;
use regex::Regex;

fn letter_frequency(input: &str) -> BTreeMap<char, i32> {
    let mut count = BTreeMap::new();
    let re = Regex::new("[^A-Za-z]").unwrap();
    for c in re.replace_all(input, "").to_lowercase().chars() {
        *count.entry(c).or_insert(0) += 1;
    }
    count
}


#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use super::letter_frequency;
    
    #[test]
    fn simpleword() {
        let answer: BTreeMap<char, i32> =
        [('a', 2),
         ('c', 1),
         ('l', 1),
         ('t', 1),
         ('u', 1)]
         .iter().cloned().collect();
         
      assert_eq!(letter_frequency("actual"), answer);
    }
    
    #[test]
    fn sequence() {
        let answer: BTreeMap<char, i32> =
        [('a', 3),
         ('b', 2),
         ('f', 1),
         ('p', 1),
         ('s', 1),
         ('t', 2),
         ('u', 1),
         ('x', 5)]
         .iter().cloned().collect();
         
      assert_eq!(letter_frequency("AaabBF UttsP xxxxx"), answer);
    }
}