fn next_combination(sub: i32) -> i32 {
    let x = sub & -sub;
    let y = sub + x;
    return (((sub & !y) / x) >> 1) | y;
}

fn list_sub() {
    let n = 5; // 全要素数
    let k = 3; // 部分集合のサイズ
    let mut bit = (1 << k) - 1;
    while bit < (1 << n) {
        for i in 0..n {
            if bit >> i & 1 == 1{
                // 部分集合に含まれる場合の処理
            }
        }

        // update
        bit = next_combination(bit);
    }
}
