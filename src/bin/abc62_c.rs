use std::cmp::{max, min};
use proconio::input;

fn main() {
    input! {
        h: u64,
        w: u64,
    };

    let mut ans = 100_000_000_000u64;

    // w / 3
    if w >= 3 {
        let ds = if w % 3 != 0 {
            h
        } else {
            0
        };
        ans = min(ans, ds);
    }

    // h / 3
    if h >= 3 {
        let ds = if h % 3 != 0 {
            w
        } else {
            0
        };
        ans = min(ans, ds);
    }

    // 1. split w, 2. split h
    for x in 1..w {

        let s1 = x * h;
        let s2 = (w - x) * (h / 2) as u64;
        let s3 = (w - x) * (h - (h / 2) as u64);
        let ds = vec![s1, s2, s3].iter().max().unwrap() - vec![s1, s2, s3].iter().min().unwrap();
        ans = min(ans, ds);
        if ans == 3 {
            println!("{} {} {}", s1, s2, s3);
        }
    }

    // 1. split h, 2. split w
    for y in 1..h {
        let s1 = y * w;
        let s2 = (h - y) * (w / 2) as u64;
        let s3 = (h - y) * (w - (w / 2) as u64);
        let ds = vec![s1, s2, s3].iter().max().unwrap() - vec![s1, s2, s3].iter().min().unwrap();
        ans = min(ans, ds);
    }

    println!("{}", ans);
}
