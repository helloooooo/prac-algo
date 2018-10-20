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
    let (n,t) = {
        let t = read_vec::<usize>();
        (t[0],t[1])
    };
    let tn = read_vec::<usize>();
    let mut ans = 0;
    for j in 0..n-1{
        if tn[j+1]-tn[j] < t {
            ans += tn[j+1]-tn[j]
        } else {
            ans += t;
        }
    }
    ans += t;
    println!("{}",ans);
}