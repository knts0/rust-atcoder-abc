use std::cmp::max;
use std::collections::HashMap;
use proconio::input;

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    return gcd(b, a % b);
}

fn lcm(a: i64, b: i64) -> i64 {
    let g = gcd(a, b);
    return a / g * b;
}

fn main() {
    input! {
        n: usize,
        t_vec: [i64; n],
    };

    let mut ans = 1_i64;
    for i in 0..n {
        ans = lcm(t_vec[i], ans);
    }
    println!("{}", ans);
}