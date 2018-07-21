use std::collections::BTreeSet;
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
fn main(){
    let s = read::<String>();
    let kk = read::<usize>();
    let mut sets = BTreeSet::new();
    for i in 0..s.len() {
        for j in 0..kk{
            if i+j >= s.len() {break;}
            sets.insert(&s[i..i+j+1]);
        }
    }
    let mut sub = Vec::new();
    for st in sets{
        sub.push(st);
    }
    sub.sort();
    println!("{}",sub[kk-1]);
}