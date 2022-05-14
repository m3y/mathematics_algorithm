use proconio::input;
use std::f64::consts::PI;

fn main() {
    input! {
        a: f64,
        b: f64,
        h: f64,
        m: f64,
    }

    // Hour hand radian
    let th = (PI / 6.0) * h + (PI / 360.0) * m;
    // Minute hand radian
    let tm = (PI / 30.0) * m;

    let va = (a * th.cos(), a * th.sin());
    let vb = (b * tm.cos(), b * tm.sin());

    // |va - vb|
    let ans = ((va.0 - vb.0).powi(2) + (va.1 - vb.1).powi(2)).sqrt();

    println!("{}", ans);
}
