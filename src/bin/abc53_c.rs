use proconio::input;

fn main() {
    input! {
        x: u64,
    };

    let res = x / 11 * 2;
    let rest = x % 11;
    if rest == 0 {
        println!("{}", res);
    } else {
        if rest > 6 {
            println!("{}", res + 2);
        } else {
            println!("{}", res + 1);
        }
    }
}
