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
    let n = read_vec::<i64>();
    let sub1 =  (1 - n[1]).abs();
    let sub2 = (n[0] - n[1]).abs();
    let ans = if sub1 < sub2 {
        sub1
    } else {
        sub2
    };
    print!("{}",ans);
}