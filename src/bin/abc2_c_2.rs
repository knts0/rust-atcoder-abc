use std::cmp::max;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        v: [(i32, i32); m],
    };

    let mut list: Vec<Vec<usize>> = vec![vec!(); n];
    for &(x, y) in &v {
        list[(x - 1) as usize].push((y - 1) as usize);
        list[(y - 1) as usize].push((x - 1) as usize);
    }

    let bit_digits = n;
    let max_bit = 1 << bit_digits;
    let mut ans = 0;
    for bit in 0..max_bit {
        let mut members = vec!();
        for i in 0..bit_digits {
            if bit >> i & 1 == 1 {
                members.push(i);
            }
        }
        let mut success = true;
        let members_len = members.len();
        for i in 0..members_len {
            for j in 0..members_len {
                if i != j {
                    if !list[members[i]].contains(&members[j]) {
                        success = false;
                    }
                }
            }
        }

        if success {
            ans = max(ans, members_len);
        }
    }
    println!("{}", ans);
}
