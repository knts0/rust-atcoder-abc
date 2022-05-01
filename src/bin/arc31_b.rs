use proconio::input;

fn dfs(nowh: usize, noww: usize, v: &mut Vec<Vec<char>>) {
    if v[nowh][noww] == 'x' {
        return;
    }

    v[nowh][noww] = 'x';
    for (dh, dw) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
        let newh = nowh as i32 + dh;
        let neww = noww as i32 + dw;
        if newh >= 0 && newh < 10 && neww >= 0 && neww < 10 {
            dfs(newh as usize, neww as usize, v);
        }
    }
}

fn main() {
    input! {
        mut v: [proconio::marker::Chars; 10],
    };

    for h in 0..10 {
        for w in 0..10 {
            let is_sea = v[h][w] == 'x';

            // 陸地にする
            v[h][w] = 'o';

            // 地図のコピー
            let mut map = v.clone();
            let mut cnt = 0;
            for h in 0..10 {
                for w in 0..10 {
                    if map[h][w] == 'o' {
                        dfs(h, w, &mut map);
                        cnt += 1;
                    }
                }
            }

            if cnt == 1 {
                println!("YES");
                return;
            }

            // 海に戻す
            if is_sea { v[h][w] = 'x' };
        }
    }
    println!("NO");

}
