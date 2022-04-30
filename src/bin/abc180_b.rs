use proconio::input;

fn main() {
    input! {
        n: usize,
        x: [f64; n],
    };

    let manhattan = x.iter().map(|v| { v.abs() }).sum::<f64>();
    let euclid = x.iter().map(|v| v.powi(2)).sum::<f64>().sqrt();
    let chebyshev = x.iter().map(|v| { v.abs() }).fold(0.0/0.0, |m, v| v.max(m));
    println!("{}\n{}\n{}", manhattan, euclid, chebyshev);
}
