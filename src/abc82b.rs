extern crate core;
use core::cmp::Ordering;
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
    let si:String = read();
    let ti:String = read();
    let mut s:Vec<char> = si.chars().collect();
    let mut t:Vec<char> = ti.chars().collect();
    s.sort();
    t.sort();
    t.reverse();
    match s.cmp(&t) {
        Ordering::Less => println!("Yes"),
        _ => println!("No"),
    };
}
