use proconio::input;
use std::cmp::{max, min};

#[derive(Debug)]
struct Vector {
    x: i64,
    y: i64,
}

impl Vector {
    fn from(p1: (i64, i64), p2: (i64, i64)) -> Self {
        Self {
            x: p2.0 - p1.0,
            y: p2.1 - p1.1,
        }
    }

    fn cross(&self, other: &Self) -> i64 {
        self.x * other.y - self.y * other.x
    }
}

fn main() {
    input! {
        a: (i64, i64),
        b: (i64, i64),
        c: (i64, i64),
        d: (i64, i64),
    };

    let ab = Vector::from(a, b);
    let ac = Vector::from(a, c);
    let ad = Vector::from(a, d);

    let cd = Vector::from(c, d);
    let ca = Vector::from(c, a);
    let cb = Vector::from(c, b);

    let c_abc = ab.cross(&ac);
    let c_abd = ab.cross(&ad);

    let c_cda = cd.cross(&ca);
    let c_cdb = cd.cross(&cb);

    if c_abc == 0 && c_abd == 0 && c_cda == 0 && c_cdb == 0 {
        let p1 = min(a, b);
        let p2 = max(a, b);
        let p3 = min(c, d);
        let p4 = max(c, d);

        if max(p1, p3) <= min(p2, p4) {
            println!("Yes");
            return;
        } else {
            println!("No");
            return;
        }
    }

    let abcd = (c_abc >= 0 && c_abd <= 0) || (c_abc <= 0 && c_abd >= 0);
    let cdab = (c_cda >= 0 && c_cdb <= 0) || (c_cda <= 0 && c_cdb >= 0);

    if abcd && cdab {
        println!("Yes");
    } else {
        println!("No");
    }
}
