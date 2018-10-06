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
    let mut v = vec![];
    for _ in 0..n{
        v.push(read::<i64>());
    } 
    let ans = v.into_iter().fold( None,|y,x| Some(match y{
        None => x,
        Some(y) => lcm(y,x),
    })).unwrap();
    println!("{}",ans);
}
fn gcd(x:i64,y:i64) -> i64{
    if x % y == 0 {
        y
    } else {
        gcd(y,x%y)
    }
}
fn lcm(x:i64,y:i64)-> i64{
    x / gcd(x, y) * y
}