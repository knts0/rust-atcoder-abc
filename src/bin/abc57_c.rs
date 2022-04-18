use std::cmp::{max, min};
use proconio::input;

fn calc_digit(mut n: u64) -> u64 {
    let mut res: u64 = 0;
    while n > 0 {
        res += 1;
        n /= 10;
    }
    res
}

fn main() {
    input! {
        n: u64,
    };

    const INF: u64 = 1 << 60;
    let mut res = INF;

    let mut i: u64 = 1;
    while i * i <= n {
        if n % i != 0 {
            i += 1;
            continue;
        }

        let b = n / i;
        let tmp: u64 = max(calc_digit(i), calc_digit(b));
        res = min(tmp, res);

        i += 1;
    }

    println!("{}", res);
}
