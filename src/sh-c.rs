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
    let nmd = read_vec::<f64>();
    let (n,m,d) = (nmd[0],nmd[1],nmd[2]);
    let diff  = if d == 0. {
        n-d
    }else {
        (n-d)*2.
    };
    println!("{:.12}",((diff/ (n*n) * (m-1.))));
}