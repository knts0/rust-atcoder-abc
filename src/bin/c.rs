use proconio::input;

fn main() {
    input! {
        n: u32,
        mut v: [u32; n],
    }

    v.sort();

    let mut expected: u32 = (n + 1) % 2;
    for (i, x) in v.into_iter().enumerate() {
        if x != expected {
            println!("0");
            return;
        }
        if (n + i as u32) % 2 == 1 {
            expected += 2;
        }
    }

    let mut res = 1;
    for i in 0..(n / 2) {
        res = 2 * res % (1_000_000_007);
    }

    println!("{}", res);
}
