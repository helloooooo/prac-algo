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
   let n = read::<i64>();
   let (a,b) = {
       let t = read_vec::<i64>();
       (t[0],t[1])
   };
   let pn = read_vec::<i64>();
   let first = pn.iter().filter(|&x| *x <= a).count();
   let second = pn.iter().filter(|&x| *x >= a+1 && *x <= b).count();
   let third = pn.iter().filter(|&x| *x >= b+1).count();
    let ans = min(first,min(second, third));
    println!("{}",ans);
}