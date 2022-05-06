use std::collections::VecDeque;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(i32, i32); m],
    };

    let mut matrix = vec![vec![false; n]; n];
    for &(a, b) in &edges {
        let a_idx = (a - 1) as usize;
        let b_idx = (b - 1) as usize;
        matrix[a_idx][b_idx] = true;
        matrix[b_idx][a_idx] = true;
    }

    if n == 1 {
        println!("IMPOSSIBLE");
        return;
    }

    // 全探索
    for i in 1..(n - 1) {
        if matrix[0][i] && matrix[i][n - 1] {
            println!("POSSIBLE");
            return;
        }
    }

    println!("IMPOSSIBLE");
}