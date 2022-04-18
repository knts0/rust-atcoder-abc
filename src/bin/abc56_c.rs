use proconio::input;

fn main() {
    input! {
        x: u64,
    };

    let mut ans = 1;

    while ans * (ans + 1) / 2 < x {
        ans += 1;
    }

    println!("{}", ans);
}
