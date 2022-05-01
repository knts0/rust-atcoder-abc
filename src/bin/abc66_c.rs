use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    };

    let mut ans = vec![0; n];
    for i in 0..n {
        if n % 2 == 0 {
            if i % 2 == 0 {
                ans[n / 2 + i / 2] = a[i];
            } else {
                ans[n / 2 - (i + 1) / 2] = a[i];
            }
        } else {
            if i % 2 == 0 {
                ans[n / 2 - i / 2] = a[i];
            } else {
                ans[n / 2 + (i + 1) / 2] = a[i];
            }
        }
    }
    let str: Vec<String> = ans.iter().map(|x| x.to_string()).collect();
    println!("{}", str.join(" "));

}
