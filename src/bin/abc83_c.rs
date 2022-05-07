use proconio::input;

fn main() {
    input! {
        x: i64,
        y: i64,
    };

    let mut i = x;
    let mut ans = 0;
    while i <= y {
        i *= 2;
        ans += 1;
    }
    println!("{}", ans);
}