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
    let n:i32 = read();
    let an:Vec<i64> = read_vec();
    let s:i64 = an.iter().sum();
    let mut x = 0;
    let mut ans = 1000000000000000000 ;
    for i in 0..(n-1){ 
        x += an[i as usize];
        if i+ 1 < n {
            ans = min(ans,(s- 2 *x).abs());
        }
    }
    println!("{}",ans);
}