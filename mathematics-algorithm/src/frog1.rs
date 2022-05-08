use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [i32; n]
    }

    let mut dp = Vec::new();

    for i in 0..n {
        if i == 0 {
            dp.push(0);
        }

        if i == 1 {
            let h1 = (h[i - 1] - h[i]).abs();
            dp.push(h1);
        }

        if i >= 2 {
            let h1 = dp[i - 1] + (h[i - 1] - h[i]).abs();
            let h2 = dp[i - 2] + (h[i - 2] - h[i]).abs();
            dp.push(if h1 > h2 { h2 } else { h1 });
        }
    }

    println!("{}", dp[n - 1]);
}
