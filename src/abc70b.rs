use std::collections::HashMap;

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
    let i : Vec<i64> = read_vec();
    let ac = vec![i[0],i[2]];
    let bd = vec![i[1],i[3]];
    let ans = if ac.iter().max().unwrap() <= bd.iter().min().unwrap() {
         bd.iter().min().unwrap() - ac.iter().max().unwrap()
    } else{
        0
    };
    println!("{}",ans);
}

