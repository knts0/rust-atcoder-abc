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

fn bit_sub() {
    let a = 0b10101100;
    let bit_digits = 8;
    let mut bit = a;
    loop {
        for i in 0..bit_digits {
            if bit >> i & 1 == 1 {
                // 列挙
            }
        }

        if bit == 0 { break };

        bit = (bit - 1) & a;
    }
}
