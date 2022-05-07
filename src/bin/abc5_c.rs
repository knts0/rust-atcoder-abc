use proconio::input;

fn main() {
    input! {
        t: usize,
        n: usize,
        a_vec: [usize; n],
        m: usize,
        b_vec: [usize; m],
    };

    let mut cust_idx = 0;
    for i in 0..n {
        let takoyaki_sec = a_vec[i];

        if cust_idx < m && takoyaki_sec <= b_vec[cust_idx] && takoyaki_sec + t >= b_vec[cust_idx] {
            cust_idx += 1;
        }
    }
    if cust_idx == m {
        println!("yes");
    } else {
        println!("no");
    }
}