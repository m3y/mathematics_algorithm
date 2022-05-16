use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n - 1],
        m: usize,
        b: [usize; m],
    }

    let mut s = vec![0; n + 1];
    for i in 2..=n {
        s[i] = s[i - 1] + a[i - 2]
    }

    let mut ans = 0;
    for j in 0..m - 1 {
        ans += (s[b[j + 1]] - s[b[j]]).abs();
    }
    println!("{}", ans);
}
