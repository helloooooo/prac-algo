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
    let (a,b,c) = {
        let t = read_vec::<i64>();
        (t[0],t[1],t[2])
    };
    let mut ans:i64 = 0;
    for _ in 0..a {
        let (h,w) = {
            let t = read_vec::<i64>();
            (t[0],t[1])
        };
        if h >= b && w >= c {
            ans += 1;
        }
    }
    println!("{}",ans);
}