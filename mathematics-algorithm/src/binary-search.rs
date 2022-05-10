use proconio::input;

fn binary_search(mut a: Vec<i32>, x: i32) -> String {
    if a.len() <= 0 {
        "No".to_string()
    } else {
        let i = a.len() / 2;
        if a[i] == x {
            "Yes".to_string()
        } else if a.len() == 1 {
            "No".to_string()
        } else if a[i] > x {
            a.split_off(i);
            binary_search(a, x)
        } else if a[i] < x {
            binary_search(a.split_off(i), x)
        } else {
            "No".to_string()
        }
    }
}

fn main() {
    input! {
        n: usize,
        x: i32,
        mut a: [i32; n],
    }

    a.sort();
    println!("{}", binary_search(a, x));

    //println!(
    //    "{}",
    //    if a.binary_search(&x).is_ok() {
    //        "Yes"
    //    } else {
    //        "No"
    //    }
    //);
}
