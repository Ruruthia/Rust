fn count_bits(n: i64) -> u32 {
    let mut ones = n;
    ones = (ones & 0x55555555) + ((ones & 0xAAAAAAAA) >> 1);
    ones = (ones & 0x33333333) + ((ones & 0xCCCCCCCC) >> 2);
    ones = (ones & 0x0F0F0F0F) + ((ones & 0xF0F0F0F0) >> 4);
    ones = (ones & 0x00FF00FF) + ((ones & 0xFF00FF00) >> 8);
    ones = (ones & 0x0000FFFF) + ((ones & 0xFFFF0000) >> 16);
    ones as u32
    // n.count_ones()
  }

