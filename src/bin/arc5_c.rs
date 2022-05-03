use std::collections::VecDeque;
use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        mut v: [proconio::marker::Chars; h],
    };
    let mut seen = vec![vec![-1; w]; h];
    let mut queue = VecDeque::new();

    let mut gh = 0;
    let mut gw = 0;
    for hcur in 0..h {
        for wcur in 0..w {
            if v[hcur][wcur] == 's' {
                queue.push_front((hcur, wcur, 0));
                seen[hcur][wcur] = 0;
            }
            if v[hcur][wcur] == 'g' {
                gh = hcur;
                gw = wcur;
            }
        }
    }

    while !&queue.is_empty() {
        let &(nowh, noww, cnt) = &queue.pop_back().unwrap();

        for (dh, dw) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
            let newh = nowh as i32 + dh;
            let neww = noww as i32 + dw;
            if newh < 0 || newh >= h as i32 || neww < 0 || neww >= w as i32 {
                continue;
            }
            let newh = newh as usize;
            let neww = neww as usize;
            let new_cnt = if v[newh][neww] == '#' { cnt + 1 } else { cnt };
            if seen[newh][neww] != -1 {
                // 塀を壊した回数がより小さい場合何もしない
                if seen[newh][neww] <= new_cnt {
                    continue;
                } else {
                    // cntで更新
                    seen[newh][neww] = new_cnt;

                    // 次の候補にする
                    queue.push_front((newh, neww, new_cnt));
                }
            } else {
                // cntで更新
                seen[newh][neww] = new_cnt;
                queue.push_front((newh, neww, new_cnt));
            }
        }
    }
    let ans = if seen[gh][gw] <= 2 { "YES" } else { "NO" };
    println!("{}", ans);

}
