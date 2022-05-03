use proconio::input;

fn main() {
    input! {
        n: usize,
        mut v: [(i32, i32); n],
    };


    for i in 0..n {
        let x = v[i].0;
        let l = v[i].1;
        v[i].0 = x - l;
        v[i].1 = x + l;
    }

    v.sort_by_key(|v| v.0);

    let mut deleted_cnt = 0;
    let mut deleted = vec![false; n];

    if n == 1 {
        println!("1");
        return;
    }

    let mut left = 0;
    while left < n - 1 {
        if deleted[left] {
            left += 1;
            continue;
        }

        let mut right = left + 1;
        // 重複がある（次のロボットアームの最小座標が、現在のアームの最大座標より小さい）
        while right < n && v[right].0 < v[left].1 {
            deleted_cnt += 1;
            // 最大座標が大きい方を削除
            if v[left].1 > v[right].1 {
                deleted[left] = true;
                break;
            } else {
                deleted[right] = true;
                right += 1;
            };
        }
        left += 1;

    }

    println!("{}", n as i32 - deleted_cnt);
}
