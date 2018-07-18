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
    let nk = read_vec::<u64>();
    let (n,k) = (nk[0],nk[1]);
    let div = n - k;
    let mut v = read_vec::<u64>();
    v.sort();
    let sum :u64 = v.iter().sum();
    let ans = sum - (0..div).fold(0,|x,y| x + v[y as usize]);
    println!("{}",ans);
}