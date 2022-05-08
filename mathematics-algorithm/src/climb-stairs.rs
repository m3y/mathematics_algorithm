use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut dp = Vec::new();

    for i in 0..=n {
        if i == 0 || i == 1 {
            dp.push(1);
        }

        if i >= 2 {
            dp.push(dp[i - 2] + dp[i - 1]);
        }
    }

    println!("{}", dp[n]);
}
