use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u64; n],
    };

    let mut ans = 0_u64;
    for i in 0..(n - 1) {
        if a[i] > a[i + 1] {
            let det = a[i] - a[i + 1];
            ans += det;
            a[i + 1] += det;
        }
    }
    println!("{}", ans);
}
