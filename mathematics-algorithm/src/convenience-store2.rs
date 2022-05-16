use proconio::input;

fn main() {
    input! {
        t: usize,
        n: usize,
        lr: [(usize, usize); n],
    }

    let mut b: Vec<i32> = vec![0; t + 1];
    for i in 0..n {
        b[lr[i].0] += 1;
        b[lr[i].1] -= 1;
    }

    let mut ans = 0;
    for j in 0..t {
        ans += b[j];
        println!("{}", ans);
    }
}
