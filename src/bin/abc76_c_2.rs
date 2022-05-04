use proconio::input;

fn main() {
    input! {
        mut s_dash: proconio::marker::Chars,
        mut t: proconio::marker::Chars,
    };

    let mut ans = vec![];
    for right in ((t.len() - 1)..s_dash.len()).rev() {
        let mut success = true;
        // tと一致させられるか
        for t_idx in 0..t.len() {
            let s_dash_idx = right - t_idx;
            if s_dash[s_dash_idx] != '?' && s_dash[s_dash_idx] != t[t.len() - 1 - t_idx] {
                success = false;
            }
        }

        if success {
            let mut ans_str = s_dash.clone();

            // tに置き換え
            for t_idx in 0..t.len() {
                let s_dash_idx = right - t_idx;
                ans_str[s_dash_idx]= t[t.len() - 1 - t_idx];
            }

            // ?をaにする
            for s_dash_idx in 0..s_dash.len() {
                if ans_str[s_dash_idx] == '?' {
                    ans_str[s_dash_idx] = 'a';
                }
            }

            ans.push(ans_str);
        }
    }
    println!("{}", &ans.iter().min().map(|v| v.iter().collect::<String>()).unwrap_or("UNRESTORABLE".to_string()));
}

