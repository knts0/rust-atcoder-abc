use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    };

    let mut cnt4 = 0_usize;
    let mut cnt_even = 0_usize;
    let mut cnt_odd = 0_usize;

    for x in a {
        if x % 4 == 0 {
            cnt4 += 1;
        } else if x % 2 == 0 {
            cnt_even += 1;
        } else {
            cnt_odd += 1;
        }
    }

    let is_yes;
    if cnt_odd > 0 {
        if cnt_even > 0 {
            is_yes = cnt_odd <= cnt4;
        } else {
            is_yes = cnt_odd <= cnt4 + 1;
        }
    } else {
        is_yes = true;
    }
    if is_yes {
        println!("Yes");
    } else {
        println!("No")
    }
}
