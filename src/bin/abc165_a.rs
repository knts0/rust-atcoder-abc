use proconio::input;

fn main() {
    input! {
        k: i32,
        a: i32,
        b: i32,
    };

    let q = b / k;

    if q * k >= a { println!("OK") } else { println!("NG") };
}
