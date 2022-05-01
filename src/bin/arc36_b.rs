use proconio::input;

fn dfs(prev: usize, i: usize, visited: &mut Vec<bool>, list: &Vec<Vec<usize>>) -> bool {
    // ループ
    if visited[i] {
        return false;
    }

    let mut success = true;
    visited[i] = true;

    for &j in &list[i] {
        if prev == j {
            continue;
        }

        let res = dfs(i, j, visited, list);
        if !res { success = false };
    }
    return success;
}

fn main() {
    input! {
        n: usize,
        m: usize,
        g: [(usize, usize); m],
    };

    let mut list: Vec<Vec<usize>> = vec![vec![]; n];

    for (u, v) in &g {
        list[u - 1].push(v - 1);
        list[v - 1].push(u - 1);
    }

    let mut ans = 0;
    let mut visited = vec![false; n];
    for i in 0..n {
        if !visited[i] {
            if dfs(100000, i, &mut visited, &list) {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
