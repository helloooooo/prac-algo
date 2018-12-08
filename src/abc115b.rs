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
    let mut v:Vec<i64> = vec![];
    for _ in 0..n{
        v.push(read::<i64>());
    }
    let max_v = v.iter().max().unwrap();
    let sum:i64 = v.iter().sum();
    let ans = sum - max_v + (max_v/2);
    println!("{}",ans);
}