use proconio::input;

fn main() {
    input! {
        s: proconio::marker::Chars,
    };

    let x: Vec<i32> = s.iter().map(|v| *v as i32 - 48).collect();

    let bit_digits = 3;
    let max_bit = 1 << bit_digits;
    for bit in 0..max_bit {
        let mut sum = x[0];
        let mut ans_str = s[0].to_string();
        for i in 0..bit_digits {
            if bit >> i & 1 == 1 {
                // +
                sum += x[i + 1];
                ans_str.push_str(&format!("+{}", x[i + 1]));
            } else {
                // -
                sum -= x[i + 1];
                ans_str.push_str(&format!("-{}", x[i + 1]));
            }
        }
        if sum == 7 {
            println!("{}=7", &ans_str);
            return;
        }
    }
}
