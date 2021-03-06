use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(f64, f64);n],
    }

    let mut ans: f64 = 10000000.0;
    for i in 0..n {
        for j in i..n {
            if i == j {
                continue;
            }
            ans = ans.min(((xy[j].0 - xy[i].0).powi(2) + (xy[j].1 - xy[i].1).powi(2)).sqrt());
        }
    }

    println!("{}", ans);
}
