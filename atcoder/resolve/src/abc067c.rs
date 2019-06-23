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
    let n:usize = read();
    let an = read_vec::<i64>();
    let sum = an.iter().sum::<i64>();
    let mut x = 0;
    let mut ans = 1e10 as i64;
    for j in 0..n-1{
        x += an[j];
        ans = min(ans,(sum-2 * x).abs());
    }
    // too late 
    // let ans = (1..n).fold(1e10 as i64,|res,i| {
    //     let x:i64 = an.iter().take(i as usize).sum();
    //     let y:i64= sum - x;
    //     min((x-y).abs(),res)
    // });
    println!("{}",ans);
}
