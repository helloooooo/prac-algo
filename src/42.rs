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
    let ab = read_vec::<i32>();
    let a1 = ab[0] + ab[1];
    let a2 = ab[0] - ab[1];
    let a3 = ab[0] * ab[1];
    let mut v = Vec::new();
    v.push(a1);
    v.push(a2);
    v.push(a3);
    let ans = v.iter().max().unwrap();
    println!("{}",ans);
}
