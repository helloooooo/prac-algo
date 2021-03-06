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
    let (n,m) = {
        let t = read_vec::<i64>();
        (t[0],t[1])
    };
    let ans = if m == 0{
        0
    } else if n > m / 2 {
        m /2
    } else {
       n + (m - n*2) /4
    };
    println!("{}",ans);
}