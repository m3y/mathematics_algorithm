use proconio::input;

fn main() {
    input! {
        n: usize,
        w: usize,
        wv: [(usize, i64); n],
    }

    let mut dp = vec![vec![-10000i64; w + 1]; n + 1];
    dp[0][0] = 0;

    for i in 1..=n {
        dp[i][0] = 0;

        for j in 0..=w {
            if j < wv[i - 1].0 {
                dp[i][j] = dp[i - 1][j]
            }

            if j >= wv[i - 1].0 {
                let a = dp[i - 1][j];
                let b = dp[i - 1][j - wv[i - 1].0] + wv[i - 1].1;
                dp[i][j] = if a > b { a } else { b };
            }
        }
    }

    println!("{}", dp[n].iter().max().unwrap());
}
