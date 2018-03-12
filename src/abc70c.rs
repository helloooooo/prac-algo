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
    let n : u64 = read();
    let mut tn : Vec<u64>= Vec::new();
    for _ in 0..n {
        tn.push(read::<u64>());
    }
    let ans = tn.into_iter().fold(None,|x,y| Some(match x {
        None => y,
        Some(x) => lcm(x,y),
    })).unwrap();
    println!("{}",ans);
}
fn gcd(a:u64,b:u64) -> u64{
    let c = a % b;
    if c == 0{
        b
    } else {
        gcd(b,c)
    }
}
fn lcm(a:u64,b:u64) -> u64{
    a / gcd(a,b) * b
}