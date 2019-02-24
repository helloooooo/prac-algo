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
    let n = read::<usize>();
    let mut sum_jp:f64 = 0.0;
    let mut sum_btc:f64 = 0.0;
    for _ in 0..n{
        let t = read_vec::<String>();
        if t[1] == "JPY" {
            sum_jp += t[0].parse::<f64>().unwrap();
        } else {
            sum_btc += t[0].parse::<f64>().unwrap();
        }
    }
    let ans = sum_jp + (sum_btc*380000.0);
    println!("{}",ans);
}