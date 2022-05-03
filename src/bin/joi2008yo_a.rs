use proconio::input;

fn main() {
    input! {
        n: i32,
    };

    let mut ans = 0;

    let mut rest = 1000 - n;
    for i in vec![500, 100, 50, 10, 5, 1] {
        if rest > 0 {
            ans += rest / i;
            rest = rest % i;
        }
    }
    println!("{}", ans);
}
