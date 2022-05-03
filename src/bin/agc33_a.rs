use std::cmp::max;
use std::collections::VecDeque;
use proconio::input;

fn bfs(
    maze: &mut Vec<Vec<char>>,
    seen: &mut Vec<Vec<i32>>,
    h: usize,
    w: usize,
) -> i32 {
    let mut queue = VecDeque::new();

    for nowh in 0..h {
        for noww in 0..w {
            if maze[nowh][noww] == '#' {
                queue.push_front((nowh, noww));
                seen[nowh][noww] = 0;
            }
        }
    }

    let mut ans = 0;
    while !&queue.is_empty() {
        let &(nowh, noww) = &queue.pop_back().unwrap();

        for (dh, dw) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
            let newh = nowh as i32 + dh;
            let neww = noww as i32 + dw;
            if newh < 0 || newh >= h as i32 || neww < 0 || neww >= w as i32 {
                continue;
            }
            let newh = newh as usize;
            let neww = neww as usize;
            if seen[newh][neww] != -1 {
                continue;
            }
            if maze[newh][neww] == '.' {
                seen[newh][neww] = seen[nowh][noww] + 1;
                ans = max(ans, seen[newh][neww]);
                maze[newh][neww] = '#';
                queue.push_front((newh, neww));
            }
        }
    }
    ans
}

fn main() {
    input! {
        h: usize,
        w: usize,
        mut v: [proconio::marker::Chars; h],
    };

    let mut seen = vec![vec![-1; w]; h];

    println!("{}", bfs(&mut v, &mut seen, h, w));
}
