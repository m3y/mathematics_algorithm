use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        p: [(usize, usize, i32); q],
    }

    let mut b: Vec<i32> = vec![0; n + 1];

    for i in 0..q {
        b[p[i].0] += p[i].2;
        if p[i].1 + 1 > n {
            continue;
        }
        b[p[i].1 + 1] -= p[i].2;
    }

    for j in 2..=n {
        if b[j] > 0 {
            print!("<");
        } else if b[j] == 0 {
            print!("=");
        } else if b[j] < 0 {
            print!(">");
        }
    }
    println!("");
}
