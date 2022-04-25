use std::cmp::max;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: u64,
        v: [(usize, u64); n],
    };

    let mut cnt = [0u64; 100010];
    let mut max_a    = 0;

    for (a, b) in v {
        cnt[a] += b;
        max_a = max(max_a, a);
    }

    let mut sum = 0u64;
    for i in 1..=max_a {
        if sum + cnt[i] >= k {
            println!("{}", i);
            return;
        }
        sum += cnt[i];
    }
}
