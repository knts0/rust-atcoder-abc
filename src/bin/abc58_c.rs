use std::cmp::min;
use proconio::input;
use std::collections::HashMap;
use std::io::Bytes;

fn main() {
    input! {
        n: usize,
        v: [String; n],
    };

    let mut min_chars = [100; 30];

    for str in &v {

        let mut char_count = [0; 30];

        for c in str.chars() {
            let cnt = c as u8 - b'a';
            char_count[cnt as usize] += 1;
        }

        for i in 0..30 {
            min_chars[i] = min(min_chars[i], char_count[i]);
        }
    }

    for i in 0..26 {
        let char = (b'a' + i) as char;
        print!("{}", char.to_string().repeat(min_chars[i as usize]));
    }
}
