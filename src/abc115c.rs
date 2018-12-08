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
fn main(){
    let (n,k) = {
        let t = read_vec::<i64>();
        (t[0],t[1])
    };
    let mut v:Vec<i64> = Vec::new();
    for _ in 0..n {
        v.push(read::<i64>());
    }
    v.sort();

    let ans = v.windows(k as usize).fold(1e10 as i64 ,|y,x|{
        let diff = x[(k-1) as usize] - x[0];
        min(diff,y)
    });
    println!("{}",ans);
}