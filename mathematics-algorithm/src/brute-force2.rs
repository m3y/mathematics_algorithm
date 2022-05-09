use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        a: [usize;n],
    }

    let mut dp = vec![vec![false; s + 1]; n + 1];
    dp[0][0] = true;

    for i in 1..=n {
        for j in 0..=s {
            if j < a[i - 1] {
                dp[i][j] = dp[i - 1][j];
            } else {
                dp[i][j] = dp[i - 1][j] || dp[i - 1][j - a[i - 1]];
            }
        }
    }

    println!("{}", if dp[n][s] { "Yes" } else { "No" });
}

// memo
// 3 11
// 2 5 9
// Yes
//
// i\j   0      1      2      3      4      5      6      7      8      9     10    11
//  0 [true, false, false, false, false, false, false, false, false, false, false, false],
//  1 [true, false, true,  false, false, false, false, false, false, false, false, false],
//  2 [true, false, true,  false, false, true,  false, true,  false, false, false, false],
//  3 [true, false, true,  false, false, true,  false, true,  false, true,  false, true]
//
// [i: 1, j: 0]		dp[0][0] = true
// [i: 1, j: 1]		dp[0][1] = false
// [i: 1, j: 2]		dp[0][2] = false
// [i: 1, j: 2]		dp[0][0] = true
// [i: 1, j: 3]		dp[0][3] = false
// [i: 1, j: 3]		dp[0][1] = false
// [i: 1, j: 4]		dp[0][4] = false
// [i: 1, j: 4]		dp[0][2] = false
// [i: 1, j: 5]		dp[0][5] = false
// [i: 1, j: 5]		dp[0][3] = false
// [i: 1, j: 6]		dp[0][6] = false
// [i: 1, j: 6]		dp[0][4] = false
// [i: 1, j: 7]		dp[0][7] = false
// [i: 1, j: 7]		dp[0][5] = false
// [i: 1, j: 8]		dp[0][8] = false
// [i: 1, j: 8]		dp[0][6] = false
// [i: 1, j: 9]		dp[0][9] = false
// [i: 1, j: 9]		dp[0][7] = false
// [i: 1, j: 10]	dp[0][10] = false
// [i: 1, j: 10]	dp[0][8] = false
// [i: 1, j: 11]	dp[0][11] = false
// [i: 1, j: 11]	dp[0][9] = false
// [i: 2, j: 0]		dp[1][0] = true
// [i: 2, j: 1]		dp[1][1] = false
// [i: 2, j: 2]		dp[1][2] = true
// [i: 2, j: 3]		dp[1][3] = false
// [i: 2, j: 4]		dp[1][4] = false
// [i: 2, j: 5]		dp[1][5] = false
// [i: 2, j: 5]		dp[1][0] = true
// [i: 2, j: 6]		dp[1][6] = false
// [i: 2, j: 6]		dp[1][1] = false
// [i: 2, j: 7]		dp[1][7] = false
// [i: 2, j: 7]		dp[1][2] = true
// [i: 2, j: 8]		dp[1][8] = false
// [i: 2, j: 8]		dp[1][3] = false
// [i: 2, j: 9]		dp[1][9] = false
// [i: 2, j: 9]		dp[1][4] = false
// [i: 2, j: 10]	dp[1][10] = false
// [i: 2, j: 10]	dp[1][5] = false
// [i: 2, j: 11]	dp[1][11] = false
// [i: 2, j: 11]	dp[1][6] = false
// [i: 3, j: 0]		dp[2][0] = true
// [i: 3, j: 1]		dp[2][1] = false
// [i: 3, j: 2]		dp[2][2] = true
// [i: 3, j: 3]		dp[2][3] = false
// [i: 3, j: 4]		dp[2][4] = false
// [i: 3, j: 5]		dp[2][5] = true
// [i: 3, j: 6]		dp[2][6] = false
// [i: 3, j: 7]		dp[2][7] = true
// [i: 3, j: 8]		dp[2][8] = false
// [i: 3, j: 9]		dp[2][9] = false
// [i: 3, j: 9]		dp[2][0] = true
// [i: 3, j: 10]	dp[2][10] = false
// [i: 3, j: 10]	dp[2][1] = false
// [i: 3, j: 11]	dp[2][11] = false
// [i: 3, j: 11]	dp[2][2] = true
