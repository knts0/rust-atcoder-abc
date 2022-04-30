fn bit() {
    let bit_digits = 10;
    let max_bit = 1 << bit_digits;
    for bit in 0..max_bit {
        for i in 0..bit_digits {
            if bit >> i & 1 == 1 {
                // true
            } else {
                // false
            }
        }
    }
}
