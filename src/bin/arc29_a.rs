use std::cmp::{max, min};
use proconio::input;

fn main() {
    input! {
        n: usize,
        t: [i32; n],
    };

    let bit_digits = n;
    let max_bit = 1 << bit_digits;

    let mut ans = 10000;
    for bit in 0..max_bit {
        let mut time1 = 0;
        let mut time2 = 0;
        for i in 0..bit_digits {
            if bit >> i & 1 == 1 {
                time1 += t[i];
            } else {
                time2 += t[i];
            }
        }
        ans = min(ans, max(time1, time2));
    }
    println!("{}", ans);
}
