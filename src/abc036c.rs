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
    let n : i64  = read();
    let mut an = Vec::new();
    for _ in 0..n{
        an.push(read::<i64>());
    }
    let mut ans =an.iter().cloned().collect::<BTreeSet<i64>>().into_iter().collect::<Vec<i64>>();
    ans.sort();
    for x in an.iter().map(|a| ans.binary_search(a).unwrap()){
        println!("{}",x );
    }
}