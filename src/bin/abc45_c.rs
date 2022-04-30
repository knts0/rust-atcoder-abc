use proconio::input;

fn main() {
    input! {
        s: String,
    };

    let s_len = s.len() as usize;
    let mut ans = 0_i64;
    let max_bit = 1 << (&s.len() - 1);
    for bit in 0..max_bit {
        let mut start = 0;
        let mut sum = 0;
        for i in 0..s_len {
            if i < s_len - 1 && bit >> i & 1 == 1 {
                sum += &s[start..=i].parse().unwrap();
                start = i + 1 as usize;
            }

            if i == s_len - 1 {
                sum += &s[start..].parse().unwrap();
            }
        }
        ans += sum;
    }
    println!("{}", ans);
}
