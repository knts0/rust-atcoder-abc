use proconio::input;

fn permutation(n: i64) -> i64 {
    if n == 0 {
        return 1;
    }
    return (n * permutation(n - 1)) % 1_000_000_007;
}

fn main() {
    input! {
        n: i32,
        m: i32,
    };

    let abs_value = (n - m).abs();
    if abs_value <= 1 {
        let mut ans = 1_i64;
        ans = permutation(n as i64);
        ans = ans * permutation(m as i64) % 1_000_000_007;
        if abs_value == 0 {
            ans = 2_i64 * ans % 1_000_000_007;
        }
        println!("{}", ans);
    } else {
        println!("0");
        return;
    }
}
