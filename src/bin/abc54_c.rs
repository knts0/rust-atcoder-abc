use proconio::input;

fn permutations(rest: Vec<i32>, cur: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
    if rest.len() == 0 {
        let mut vector = vec![1];
        vector.append(cur);
        ans.push(vector);
        return;
    }

    for i in 0..rest.len() {
        let mut new_cur = cur.clone();
        new_cur.push(rest[i]);
        let mut new_rest = rest.clone();
        new_rest.remove(i);
        permutations(new_rest, &mut new_cur, ans);
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(usize, usize); m],
    };

    let mut is_connected = vec![vec![false; n]; n];

    for (a, b) in &edges {
        is_connected[a - 1][b - 1] = true;
        is_connected[b - 1][a - 1] = true;
    }

    let mut ans = vec![];
    let rest: Vec<i32> = (2..=n).into_iter().map(|v| v as i32).collect();
    permutations(rest, &mut vec![], &mut ans);

    let mut ans_cnt = 0;
    for v in &ans {
        let mut is_ok = true;
        for i in 0..(n - 1) {
            // 辺がない場合NG
            // 0-indexedにする
            let v_i = (v[i] - 1) as usize;
            let v_i1 = (v[i + 1] - 1) as usize;
            if !is_connected[v_i][v_i1] {
                is_ok = false;
                break;
            }
        }
        if is_ok {
            ans_cnt += 1;
        }
    }

    println!("{}", ans_cnt);

}
