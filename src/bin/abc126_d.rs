use proconio::input;

fn dfs(
    adj_list: &Vec<Vec<(usize, i32)>>,
    color: &mut Vec<i32>,   // -1の場合色が未確定
    v: usize,
    cur: i32,   // 現在の色
) {
    color[v] = cur;

    for &(next_v, w) in &adj_list[v] {
        // すでに色確定済み
        if color[next_v] != -1 {
            continue;
        }

        // 偶数なら同じ色
        if w % 2 == 0 {
            dfs(adj_list, color, next_v, cur)
        } else {
            dfs(adj_list, color, next_v, 1 - cur)
        }
    }
}

fn main() {
    input! {
        n: usize,
        edges: [(usize, usize, i32); (n - 1) as usize],
    };

    let mut adj_list = vec![vec![]; n];

    for &(a, b, w) in &edges {
        adj_list[(a - 1) as usize].push(((b - 1) as usize, w));
        adj_list[(b - 1) as usize].push(((a - 1) as usize, w));
    }

    let mut color = vec![-1; n];

    dfs(&adj_list, &mut color, 0, 0);

    for &c in &color {
        println!("{}", c);
    }
}
