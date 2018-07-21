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
    let ab = read_vec::<i64>();
    let (a,b) = (ab[0],ab[1]);
    let plus = a + b;
    let div = a * b;
    let ans = if plus == 15 {
        "+".to_string()
    } else if div == 15 {
        "*".to_string()
    } else {
        "x".to_string()
    };
    println!("{}",ans);
}