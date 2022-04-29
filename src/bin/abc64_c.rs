use std::cmp::{max, min};
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    };

    let mut colors_exist = [false; 8];
    let mut high_rate_cnt = 0;

    for i in a {
        if i < 3200 {
            colors_exist[(i / 400) as usize] = true;
        } else {
            high_rate_cnt += 1;
        }
    }

    let mut kind_cnt = 0;
    for x in &colors_exist {
        if *x {
            kind_cnt += 1;
        }
    }

    let min_kind = max(kind_cnt, 1);
    let max_kind = kind_cnt + high_rate_cnt;
    println!("{} {}", min_kind, max_kind);
}
