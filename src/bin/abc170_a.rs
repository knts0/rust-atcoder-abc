use proconio::input;

fn main() {
    input! {
        v: [i32; 5],
    };

    for i in 0..5 {
        if v[i] == 0 {
            println!("{}", i + 1);
            return;
        }
    }
}
