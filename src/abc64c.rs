use std::collections::HashSet;
use std::cmp::min;
fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}

fn read_vec2<T: std::str::FromStr>(n: u32) -> Vec<Vec<T>> {
    (0..n).map(|_| read_vec()).collect()
}
fn main() {
    let n: i32 = read();
    let an:Vec<i32> = read_vec();
    let under_red = an.iter().filter(|&&x| x < 3200).map(|x| x / 400).collect::<HashSet<_>>().len();
    let over_red = an.iter().filter(|&&x| x >= 3200).count();
    let min_ans = if  under_red == 0 {
        min(1,over_red)
    } else {
        under_red
    };
    let max_ans = under_red + over_red;
    println!("{} {}",min_ans,max_ans);
}