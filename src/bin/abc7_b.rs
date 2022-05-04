use proconio::input;

fn main() {
    input! {
        v: proconio::marker::Chars,
    };

    if v.len() > 1 {
        println!("{}", &v[0..(v.len() - 1)].iter().collect::<String>());
    } else {
        if v[0] == 'a' {
            println!("-1");
        } else {
            let num = v[0] as u8 - 1;
            let ans_char = num as char;
            println!("{}", ans_char);
        }
    }
}
