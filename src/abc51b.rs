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
fn main() {
    let ks = read_vec::<i32>();
    let mut ans = 0;
    for j in 0..ks[0]+1{
        for k in 0..ks[0]+1{
            let z = ks[1] - j - k;
            if 0 <= z && z <= ks[0] { ans += 1;}
        }
    }
    println!("{}",ans );
}