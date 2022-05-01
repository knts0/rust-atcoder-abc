use std::cmp::min;
use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        r: usize,
        c: usize,
        sy: usize,
        sx: usize,
        gy: usize,
        gx: usize,
        maze: [proconio::marker::Chars; r],
    };

    let mut min_dest = vec![vec![-1; c + 1]; r + 1];
    let mut queue = VecDeque::new();
    queue.push_front((sy, sx));
    min_dest[sy][sx] = 0;

    while !&queue.is_empty() {
        let &(y, x) = &queue.pop_back().unwrap();

        // 4方向に探索
        for &(dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
            let next_y = (y as i32 + dy) as usize;
            let next_x = (x as i32 + dx) as usize;

            if next_y > 1 && next_y < r && next_x > 1 && next_x < c && maze[next_y - 1][next_x - 1] != '#' && min_dest[next_y][next_x] == -1 {
                &queue.push_front((next_y, next_x));
                min_dest[next_y][next_x] = min_dest[y][x] + 1;
            }
        }
    }
    println!("{}", min_dest[gy][gx]);

}
