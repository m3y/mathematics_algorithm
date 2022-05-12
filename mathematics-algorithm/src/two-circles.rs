use proconio::derive_readable;
use proconio::input;

#[derive_readable]
#[derive(Debug)]
struct Circle {
    x: f64,
    y: f64,
    r: f64,
}

fn main() {
    input! {
        a : Circle,
        b : Circle,
    }

    let ab = ((b.x - a.x).powi(2) + (b.y - a.y).powi(2)).sqrt();

    if (a.r - b.r).abs() > ab {
        println!("1");
        return;
    }

    if (a.r - b.r).abs() == ab {
        println!("2");
        return;
    }

    if a.r + b.r > ab {
        println!("3");
        return;
    }

    if a.r + b.r == ab {
        println!("4");
        return;
    }

    if a.r + b.r < ab {
        println!("5");
        return;
    }
}
