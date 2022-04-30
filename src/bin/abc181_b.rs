use proconio::input;

fn main() {
    input! {
        n: usize,
        v: [(u64, u64); n],
    };

    let mut ans = 0_u64;
    for &(a, b) in &v {
        ans += b * (b + 1) / 2 - (a - 1) * a / 2;
    }
    println!("{}", ans);
}
