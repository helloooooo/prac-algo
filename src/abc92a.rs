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
fn  main() {
    let a = read::<i32>();
    let b = read::<i32>();
    let c = read::<i32>();
    let d = read::<i32>();
    let e = if a > b {
        b
    } else {
        a
    };
    let f = if c > d{
        d 
    } else {
        c
    };
    println!("{}",e +f );
}