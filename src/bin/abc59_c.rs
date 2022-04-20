use std::cmp::min;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    };

    let mut ans1: i64 = 0;
    let mut sum1: i64 = 0;
    // 偶数番目が+　奇数番目が-
    for i in 0..n {
        sum1 += a[i] as i64;
        if i % 2 == 0 && sum1 <= 0 {
            ans1 += 1 - sum1;
            sum1 = 1;
        }

        if i % 2 == 1 && sum1 >= 0 {
            ans1 += sum1 + 1;
            sum1 = -1;
        }
    }

    let mut ans2: i64 = 0;
    let mut sum2: i64 = 0;
    // 偶数番目が-　奇数番目が+
    for i in 0..n {
        sum2 += a[i] as i64;
        if i % 2 == 0 && sum2 >= 0 {
            ans2 += sum2 + 1;
            sum2 = -1;
        }

        if i % 2 == 1 && sum2 <= 0 {
            ans2 += -sum2 + 1;
            sum2 = 1;
        }
    }

    println!("{}", min(ans1, ans2));
}
