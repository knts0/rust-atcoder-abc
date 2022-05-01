use std::cmp::min;
use proconio::input;

fn main() {
    input! {
        d: usize,
        g: i64,
        v: [(i64, i64); d],
    };

    let bit_digits = d;
    let max_bit = 1 << bit_digits;
    let mut ans = 1_000_000;
    for bit in 0..max_bit {
        let mut total_point = 0_i64;
        let mut problems = 0;

        // ボーナスがつく場合の総和を先にもとめる
        for i in 0..bit_digits {
            let rev_i = (bit_digits - 1) - i;
            let unit_point = (rev_i as i64 + 1_i64) * 100_i64;
            let p = v[rev_i].0;
            let c = v[rev_i].1;

            if bit >> i & 1 == 1 {
                // ボーナスが付く
                total_point += p * unit_point + c;
                problems += p;
            }
        }

        if total_point >= g {
            ans = min(ans, problems);
            continue;
        }

        // 残り（ボーナス付かない）を貪欲で回す
        for i in 0..bit_digits {
            let rev_i = (bit_digits - 1) - i;
            let unit_point = (rev_i as i64 + 1_i64) * 100_i64;
            let p = v[rev_i].0;

            if bit >> i & 1 == 0 {
                let q = (g - total_point + unit_point - 1) / unit_point;
                if q <= p - 1 {
                    ans = min(ans, problems + q);
                    break;
                }
                total_point += unit_point * (p - 1);
                problems += p - 1;
            }
        }
    }
    println!("{}", ans);
}
