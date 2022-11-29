fn spin_words(words: &str) -> String {
    words.split_whitespace().map(|s| {
        if s.len() >= 5 {
            s.chars().rev().collect()
        }
        else {
            s.to_string()
        }
    }).collect::<Vec<String>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(spin_words("Welcome"), "emocleW");
        assert_eq!(spin_words("Hey fellow warriors"), "Hey wollef sroirraw");
        assert_eq!(spin_words("This is a test"), "This is a test");
        assert_eq!(spin_words("This is another test"), "This is rehtona test");
        assert_eq!(spin_words("You are almost to the last test"), "You are tsomla to the last test");
        assert_eq!(spin_words("Just kidding there is still one more"), "Just gniddik ereht is llits one more");
        assert_eq!(spin_words("Seriously this is the last one"), "ylsuoireS this is the last one");
    }

    #[test]
    fn my_test_1() {
        assert_eq!(spin_words("Hi, hello there!"), "Hi, olleh !ereht");
    }

    #[test]
    fn my_test_2() {
        assert_eq!(spin_words("Are the  double  whitespaces a problem?"), "Are the elbuod secapsetihw a ?melborp");
    }

    #[test]
    fn my_test_3() {
        assert_eq!(spin_words(""), "");
    }

    #[test]
    fn my_test_4() {
        assert_eq!(spin_words(" Are spaces at the beginning and end a problem? "), "Are secaps at the gninnigeb and end a ?melborp");
    }

    #[test]
    fn my_test_5() {
        assert_eq!(spin_words("Last test!!!"), "Last !!!tset");
    }
}
