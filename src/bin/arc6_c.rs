use std::cmp::min;
use proconio::input;

fn main() {
    input! {
        n: usize,
        w_vec: [i32; n],
    };

    let mut pile_top = vec![];

    for w in w_vec {
        let mut min_top = 1_000_000;
        let mut next_idx = -1_i32;

        for i in 0..pile_top.len() {
            // 上に積み上げ可能な場合
            if w <= pile_top[i] && min_top > pile_top[i] {
                min_top = pile_top[i];
                next_idx = i as i32;
            }
        }
        if next_idx == -1 {
            pile_top.push(w);
        } else {
            pile_top[next_idx as usize] = w;
        }
    }

    println!("{}", pile_top.len());
}