use proconio::input;

fn main() {
    input! {
        s: String,
    };

    let s_len = s.len() as usize;
    let mut ans = 0_i64;
    let max_bit = 1 << (&s.len() - 1);
    for bit in 0..max_bit {
        let mut now = s[0].parse().unwrap();
        let mut sum = 0;
        for i in 0..(s_len - 1) {
            if bit >> i & 1 == 1 {
                sum += now;
                now = 0;
            } else {
                now = now * 10 + s[i + 1];
            }
        }
        ans += sum;
    }
    println!("{}", ans);
}
