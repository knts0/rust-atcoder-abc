use std::collections::VecDeque;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(i32, i32); m],
    };

    let mut adj_list: Vec<Vec<usize>> = vec![vec![]; n];
    for &(a, b) in &edges {
        let a_idx = (a - 1) as usize;
        let b_idx = (b - 1) as usize;
        adj_list[a_idx].push(b_idx);
        adj_list[b_idx].push(a_idx);
    }

    // 最短距離
    let mut dist = vec![-1; n];

    let mut queue = VecDeque::new();
    queue.push_front(0);

    dist[0] = 0;

    while !&queue.is_empty() {
        let &v = &queue.pop_back().unwrap();

        for &next_v in &adj_list[v] {
            if dist[next_v] > -1 {
                continue;
            }
            dist[next_v] = dist[v] + 1;
            queue.push_front(next_v);
        }
    }

    if dist[n - 1] > -1 && dist[n - 1] <= 2 {
        println!("POSSIBLE");
    } else {
        println!("IMPOSSIBLE");
    }
}