use std::cmp::min;
use proconio::input;

fn main() {
    input! {
        a: [i32; 4],
    };

    let mut ans = 1000;
    for &x in &a {
        ans = min(x, ans);
    }
    println!("{}", ans);
}
