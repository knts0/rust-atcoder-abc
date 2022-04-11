use proconio::input;



fn main() {
    input! {
        n: usize,
    };

    if n == 1 {
        println!("1");
        return
    }

    let mut divisor_cnt: [i32; 2000] = [0; 2000];

    for i in 2..=n {
        let mut num = i;
        for j in 2..=i {
            while num % j == 0 {
                divisor_cnt[j] += 1;
                num /= j;
            }
        }
    }

    let mut res: u64 = 1;
    for i in 2..=n {
        if divisor_cnt[i] != 0 {
            res = res * (divisor_cnt[i] + 1) as u64 % 1_000_000_007 as u64;
        }
    }

    println!("{}", res);
}
