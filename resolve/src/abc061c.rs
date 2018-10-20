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
    let (n,k) = {
        let t = read_vec::<i64>();
        (t[0],t[1])
    };
    let mut v = vec![];
    for _ in 0..n{
        let (a,b) = {
            let t = read_vec::<i64>();
            (t[0],t[1])
        };
        v.push((a,b));
    }
    v.sort();
    let mut ans = 0;
    let mut sum = 0;
    for  &(a,b) in &v{
        if sum >= k{
            break;
        }
        sum += b;
        ans = a;
    }
    println!("{}",ans);
}