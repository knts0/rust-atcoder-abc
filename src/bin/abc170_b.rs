use proconio::input;

fn main() {
    input! {
        x: i32,
        y: i32,
    };

    for tsuru in 0..=x {
        let kame = x - tsuru;
        if tsuru * 2 + kame * 4 == y {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
