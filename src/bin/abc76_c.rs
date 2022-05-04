// これは嘘解法だったが残しておく

use proconio::input;

fn main() {
    input! {
        mut s_dash: proconio::marker::Chars,
        mut t: proconio::marker::Chars,
    };

    // tを反転させる
    let t_rev: Vec<char> = t.into_iter().rev().collect();
    let t_len = t_rev.len();

    // 後ろから走査し、tが当てはめられるか調べる
    let mut i = s_dash.len() as i32 - 1;
    while i >= 0 {
        let mut is_match = true;
        for j in 0..t_len {
            if (i - j as i32) < 0 {
                is_match = false;
                break;
            }
            let s_dash_idx = (i - j as i32) as usize;
            if s_dash[s_dash_idx] != '?' && s_dash[s_dash_idx] != t_rev[j] {
                is_match = false;
                break;
            }
        }

        if is_match {
            // 文字列を変更
            for ti in 0..t_len {
                let s_dash_idx = (i - ti as i32) as usize;
                s_dash[s_dash_idx] = t_rev[ti];
            }
            // ?をaで埋める
            for si in 0..s_dash.len() {
                if s_dash[si] == '?' {
                    s_dash[si] = 'a';
                }
            }
            println!("{}", s_dash.iter().collect::<String>());
            return;
        } else {
            i -= 1;
        }
    }
    println!("UNRESTORABLE");
}
