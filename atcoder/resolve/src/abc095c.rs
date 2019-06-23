use std::cmp::{min,max};
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
    let (a,b,c,x,y) = {
        let t = read_vec::<i64>();
        (t[0],t[1],t[2],t[3],t[4])
    };
    let c = c * 2;
    let mut ans =  1e9 as i64 ;
    for i in 0..100001{
        ans  = min((max(0,x-i)*a + max(0,y-i)*b + i*c),ans);
    }
    println!("{}",ans );
}