use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        v: [(i32, i32); m],
    };
    let mut list: Vec<Vec<i32>> = vec![vec!(); n + 1_usize];
    for &(x, y) in &v {
        list[x as usize].push(y);
        list[y as usize].push(x);
    }

    let mut ans: Vec<Vec<i32>> = vec![vec!()];
    for i in 1..=n {
        let mut solo = true;
        for j in &mut ans {
            let mut success = true;
            for elem in j.iter() {
                if !&list[i].iter().any(|&v| {v == *elem}) {
                    success = false;
                }
            }
            if success {
                j.push(i as i32);
                solo = false;
                break;
            }
        }
        if solo {
            ans.push(vec!(i as i32));
        };
    }
    println!("{:?}", &ans.iter().map(|v| {v.len()}).max().unwrap());
}
