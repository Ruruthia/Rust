fn dna_strand(dna: &str) -> String {
    let mut complementary_dna = String::new();
    for symbol in dna.chars(){
        let complementary = match symbol {
            'A' => 'T',
            'T' => 'A',
            'C' => 'G',
            'G' => 'C',
            _ => '?',
        };     
        complementary_dna.push(complementary);
    }
    complementary_dna
}

#[cfg(test)]
mod tests {
    use super::dna_strand;

    fn dotest(s: &str, expected: &str) {
        let actual = dna_strand(s);
        assert!(actual == expected, 
            "With dna = \"{s}\"\nExpected \"{expected}\" but got \"{actual}\"")
    }
    
    #[test]
    fn fixed_tests() {
        dotest("AAAA","TTTT");
        dotest("ATTGC","TAACG");
        dotest("GTAT","CATA");
    }
    
    #[test]
    fn my_test_1() {
        dotest("TATA","ATAT");
    }   
    
    #[test]
    fn my_test_2() {
        dotest("AGTC","TCAG");
    }  
    #[test]
    fn my_test_3() {
        dotest("AGAT","TCTA");
    }  
    
    #[test]
    fn my_test_4() {
        dotest("GCCT","CGGA");
    }  
    
    #[test]
    fn my_test_5() {
        dotest("ACGACG","TGCTGC");
    }      
}