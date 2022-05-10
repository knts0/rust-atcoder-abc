use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a_vec: [i64; n],
    };

    a_vec.sort();
    a_vec.reverse();

    let mut max1 = 0;
    let mut max1_c = 0;
    let mut max2 = 0;
    let mut max2_c = 0;

    let mut i = 0;
    while i < n {
        let num = a_vec[i];
        let mut j = i;
        let mut cnt = 0;
        while j < n && num == a_vec[j] {
            j += 1;
            cnt += 1;
        }
        if cnt >= 2 {
            if max1 == 0 {
                max1 = num;
                max1_c = cnt;
            } else if max2 == 0 {
                max2 = num;
                max2_c = cnt;
            }
        }
        i = j;
    }
    if max1_c >= 4 {
        println!("{}", max1 * max1);
    } else if max1_c >= 2 && max2_c >= 2 {
        println!("{}", max1 * max2);
    } else {
        println!("0");
    }
}