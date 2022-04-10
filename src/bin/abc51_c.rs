use proconio::input;

fn main() {
    input! {
        sx: i32,
        sy: i32,
        tx: i32,
        ty: i32,
    };

    let dx = (tx - sx) as usize;
    let dy = (ty - sy) as usize;

    // 1
    print!("{}", "U".repeat(dy));
    print!("{}", "R".repeat(dx));
    print!("{}", "D".repeat(dy));
    print!("{}", "L".repeat(dx));

    // 2
    print!("L");
    print!("{}", "U".repeat(dy + 1));
    print!("{}", "R".repeat(dx + 1));
    print!("DR");
    print!("{}", "D".repeat(dy + 1));
    print!("{}", "L".repeat(dx + 1));
    println!("U");

}
