use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [i32; n],
    };

    const MAX_SUM: usize = 10200;

    let mut dp = vec![vec![false; MAX_SUM]; n + 1];

    dp[0][0] = true;

    for i in 0..n {
        for sum in 0..10001 {
            if dp[i][sum] {
                dp[i + 1][sum] = true;
                dp[i + 1][sum + s[i] as usize] = true;
            }
        }
    }

    let mut ans = 0;
    for sum in 0..MAX_SUM {
        if dp[n][sum] {
            if sum % 10 != 0 {
                ans = sum;
            }
        }
    }
    println!("{}", ans);
}
