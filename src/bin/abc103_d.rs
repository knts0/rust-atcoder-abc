use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut v: [(i32, i32); m],
    };

    v.sort_by_key(|num| { num.1 });

    let mut ans = 0;
    let mut deleted = -1;
    for &(a, b) in &v {
        if deleted <= a {
            ans += 1;
            deleted = b;
        }
    }
    println!("{}", ans);
}
