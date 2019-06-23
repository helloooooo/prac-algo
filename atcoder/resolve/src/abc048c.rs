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
    let (n,x) = {
        let t = read_vec::<usize>();
        (t[0],t[1])
    };
    let mut an = read_vec::<i64>();
    let mut ans = 0;
    for j in 0..n-1{
        let sum = an[j] + an[j+1];
        if sum > x as i64{
            ans +=  sum - x as i64;
            an[j+1] -= std::cmp::min(sum -x as i64,an[j+1]);
            an[j] -= (sum - x as i64) - std::cmp::min(sum -x as i64,an[j+1]);
        }
    }
    println!("{}",ans);
}
