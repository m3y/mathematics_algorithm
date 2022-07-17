use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut f = vec![0; n + 1];

    for i in 1..=n {
        for j in (i..=n).step_by(i) {
            f[j] += 1
        }
    }

    let mut ans = 0;
    for i in 1..=n {
        ans += i * f[i]
    }

    println!("{:?}", ans);
}
