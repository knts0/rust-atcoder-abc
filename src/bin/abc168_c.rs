use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
        h: f64,
        m: f64,
    };

    let h_angl = 30.0 * h + 0.5 * m;
    let m_angl = 6.0 * m;

    let ans = a * a + b * b - 2.0 * a * b * ((h_angl - m_angl) / 360.0 * 2.0 * std::f64::consts::PI).cos();

    println!("{}", ans.sqrt());
}
