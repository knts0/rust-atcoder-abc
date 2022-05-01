use std::collections::VecDeque;
use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [proconio::marker::Chars; h],
    };

    let mut black = 0;
    for row in &s {
        for c in row {
            if *c == '#' {
                black += 1;
            }
        }
    }

    let mut min_dest : Vec<Vec<i32>> = vec![vec![-1; w]; h];
    let mut queue = VecDeque::new();
    queue.push_front((0, 0));
    min_dest[0][0] = 0;

    while !&queue.is_empty() {
        let &(y, x) = &queue.pop_back().unwrap();

        // 4方向に探索
        for &(dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
            let next_y = y as i32 + dy;
            let next_x = x as i32 + dx;
            if next_y < 0 || next_x < 0 || next_y >= h as i32 || next_x >= w as i32 {
                continue;
            }
            let next_y = next_y as usize;
            let next_x = next_x as usize;

            if s[next_y][next_x] != '#' && min_dest[next_y][next_x] == -1 {
                &queue.push_front((next_y, next_x));
                min_dest[next_y][next_x] = min_dest[y][x] + 1;
            }
        }
    }
    if min_dest[h - 1][w - 1] == -1 {
        println!("-1");
    } else {
        println!("{}", h as i32 * w as i32 - min_dest[h - 1][w - 1] - 1 - black);
    }
}
