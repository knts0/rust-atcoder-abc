use proconio::input;

fn main() {
    input! {
        n: i32,
        x: i32,
        t: i32,
    };

    let cnt = (n + x - 1) / x;

    println!("{}", cnt * t);
}
