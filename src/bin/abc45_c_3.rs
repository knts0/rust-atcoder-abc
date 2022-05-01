use proconio::input;

fn calc(i: usize, sum: i64, prev: i64, s: &Vec<char>) -> i64 {
    let now = s[i] as i64 - 48_i64;
    if i == s.len() - 1 {
        return prev * 10_i64 + sum + now;
    }

    // nowを使う
    let sum1 = calc(i + 1, sum + prev * 10_i64 + now, 0_i64, s);
    let sum2 = calc(i + 1, sum, prev * 10_i64 + now, s);
    return sum1 + sum2;
}

fn main() {
    input! {
        s: proconio::marker::Chars,
    };

    println!("{}", calc(0, 0_i64, 0_i64, &s));

}
