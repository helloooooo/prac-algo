use std::cmp::min;

fn main() {
    let mut res = 0;
    let coin = vec![3, 2, 1, 3, 0, 2];
    let mut ans = 620;
    let v: Vec<i32> = vec![1, 5, 10, 50, 100, 500];
    for i in 0..6 {
        let t = min(ans / v[5 - i], coin[5 - i]);
        ans -= t * v[5 - i];
        res += t;
    }
    println!("{}", res);
}
