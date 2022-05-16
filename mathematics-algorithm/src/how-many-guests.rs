use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize;n],
        lr: [(usize, usize); q],
    }

    let mut b = vec![0; n + 1];

    for i in 1..=n {
        b[i] = b[i - 1] + a[i - 1];
    }

    for j in 0..q {
        println!("{}", b[lr[j].1] - b[lr[j].0 - 1]);
    }
}
