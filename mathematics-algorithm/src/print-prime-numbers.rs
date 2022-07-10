use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut v = vec![true; n + 1];

    for i in 2..=((n as f64).sqrt() as usize) {
        if v[i] != true {
            continue;
        }

        for j in i + 1..=n {
            if j % i == 0 {
                v[j] = false
            }
        }
    }

    for i in 2..=n {
        if v[i] {
            print!("{} ", i)
        }
    }
    println!("")
}
