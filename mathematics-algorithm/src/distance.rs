use proconio::input;

fn main() {
    input! {
        a: (f64, f64),
        b: (f64, f64),
        c: (f64, f64),
    }

    let ba = (a.0 - b.0, a.1 - b.1);
    let bc = (c.0 - b.0, c.1 - b.1);
    let ca = (a.0 - c.0, a.1 - c.1);
    let cb = (b.0 - c.0, b.1 - c.1);

    // ba*bc
    let s_ba_bc = ba.0 * bc.0 + ba.1 * bc.1;
    if s_ba_bc.is_sign_negative() {
        // |ba|
        println!("{}", (ba.0 * ba.0 + ba.1 * ba.1).sqrt());
        return;
    }

    // ca*cb
    let s_ca_cb = ca.0 * cb.0 + ca.1 * cb.1;
    if s_ca_cb.is_sign_negative() {
        // |ca|
        println!("{}", (ca.0 * ca.0 + ca.1 * ca.1).sqrt());
        return;
    }

    // d * |bc| = |ba * bc|
    println!(
        "{}",
        (ba.0 * bc.1 - ba.1 * bc.0).abs() / (bc.0 * bc.0 + bc.1 * bc.1).sqrt()
    );
}
