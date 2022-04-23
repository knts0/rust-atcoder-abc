use proconio::input;

fn main() {
    input! {
        n: usize,
        t: i32,
        v: [i32; n],
    };

    let mut ans = 0;
    for i in 0..(n - 1) {
        let d = v[i + 1] - v[i];
        if d < t {
            ans += d;
        } else {
            ans += t;
        }
    }
    ans += t;
    println!("{}", ans);
}
