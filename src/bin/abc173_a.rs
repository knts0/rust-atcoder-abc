use std::cmp::max;
use proconio::input;

fn main() {
    input! {
        x: i32,
    };

    if x == 0 { println!("1") } else { println!("0") };

    // 別解
    // 1 - x
    // not x
    // x xor 1
}
