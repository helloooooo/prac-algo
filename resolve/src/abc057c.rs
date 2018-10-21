use std::cmp::{max,min};
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
    let n:i64 = read();
    let m:i64 = (n as f64).sqrt() as i64;
    let mut ans = 10;
    for j in 1..(m+ 1){
        if n % j == 0 {
            ans = min(ans,max(get_length(j,0),get_length(n/j,0)));
        }
    }
    println!("{}",ans );
}
fn get_length(x:i64,cnt:i64) -> i64{
    if x <= 0 {
        cnt
    } else {
        get_length(x/10, cnt+1)
    }
    // let mut cnt = 0;
    // while x > 0 {
    //     cnt += 1;
    //     x /= 10;
    // }
    // cnt
}