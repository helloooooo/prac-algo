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
    let an:Vec<i64> = read_vec();
    let ans = (-100..101).fold(1e10 as i64,|y,x|{
        let sum:i64 = an.iter().map(|z| (z-x).abs().pow(2) ).sum();
        std::cmp::min(sum,y)
    });
    println!("{}",ans);
}
