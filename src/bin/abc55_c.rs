use proconio::input;

fn main() {
    input! {
        n: u64,
        mut m: u64,
    };
    if n <= m / 2 {
        let mut res: u64 = n;
        res += (m - n * 2) / 4;
        println!("{}", res);
    } else {
        println!("{}", m / 2);
    }
}
