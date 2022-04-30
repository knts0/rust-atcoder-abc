use proconio::input;

fn main() {
    input! {
        n: usize,
        d: i64,
        v: [(i64, i64); n],
    };

    let mut ans = 0;
    for &(x, y) in &v {
        let dist = x * x + y * y;
        if dist <= d * d {
            ans += 1;
        }
    }

    println!("{}", ans);
}
