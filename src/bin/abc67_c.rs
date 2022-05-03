use std::cmp::{max, min};
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };

    let sum: i64 = a.iter().sum();
    let mut cur_abs = 1_000_000_000_000_i64;
    let mut cur_sum = 0;
    for i in 0..(n - 1) {
        cur_sum += a[i];
        let target = (sum - 2_i64 * cur_sum).abs();
        cur_abs = min(cur_abs, target);
    }
    println!("{}", cur_abs);
}
