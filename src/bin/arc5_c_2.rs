// 0-1 BFSで解く

use std::collections::VecDeque;
use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        mut field: [proconio::marker::Chars; h],
    };
    let mut dist = vec![vec![1_000_000; w]; h];
    let mut queue = VecDeque::new();

    let mut gh = 0;
    let mut gw = 0;
    for hcur in 0..h {
        for wcur in 0..w {
            if field[hcur][wcur] == 's' {
                queue.push_front((hcur, wcur));
                dist[hcur][wcur] = 0;
            }
            if field[hcur][wcur] == 'g' {
                gh = hcur;
                gw = wcur;
            }
        }
    }

    while !&queue.is_empty() {
        let &(nowh, noww) = &queue.pop_front().unwrap();

        for (dh, dw) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
            let newh = nowh as i32 + dh;
            let neww = noww as i32 + dw;
            if newh < 0 || newh >= h as i32 || neww < 0 || neww >= w as i32 {
                continue;
            }
            let newh = newh as usize;
            let neww = neww as usize;

            // 次が塀でない場合（コスト0）、先頭にpush
            if field[newh][neww] != '#' {
                if dist[newh][neww] > dist[nowh][noww] {
                    queue.push_front((newh, neww));
                    dist[newh][neww] = dist[nowh][noww];
                }
            } else {
                if dist[newh][neww] > dist[nowh][noww] + 1 {
                    queue.push_back((newh, neww));
                    dist[newh][neww] = dist[nowh][noww] + 1;
                }
            }
        }
    }

    if dist[gh][gw] <= 2 {
        println!("YES");
    } else {
        println!("NO");
    }
}

