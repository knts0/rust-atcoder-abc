use proconio::input;

fn dfs(ph: usize, pw: usize, h: usize, w: usize, v: &mut Vec<Vec<char>>) -> bool {
    if v[ph][pw] == 'g' {
        return true;
    }

    if v[ph][pw] == '#' {
        return false;
    }

    v[ph][pw] = '#';
    let mut res = false;
    for (dh, dw) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
        let newh = ph as i32 + dh;
        let neww = pw as i32 + dw;
        if newh >= 0 && newh < (h as i32) && neww >= 0 && neww < (w as i32) {
            if dfs(newh as usize, neww as usize, h, w, v) {
                res = true;
            };
        }
    }
    v[ph][pw] = '.';
    return res;
}

fn main() {
    input! {
        h: usize,
        w: usize,
        mut v: [proconio::marker::Chars; h],
    };
    for h_now in 0..h {
        for w_now in 0..w {
            if v[h_now][w_now] == 's' {
                if dfs(h_now, w_now, h, w, &mut v) {
                    println!("Yes");
                } else {
                    println!("No");
                }
            }
        }
    }
}
