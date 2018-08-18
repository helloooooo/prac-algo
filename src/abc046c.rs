use std::cmp::max;
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
    let n : i64 = read();
    let ta = read_vec2(n as u32);
    // let mut ans = (ta[0][0],ta[0][1]);
    let mut ans = (1,1);
    for j in 0..n {
        let t:i64 = ta[j as usize][0];
        let a:i64 = ta[j as usize][1];
        let o =  max((ans.0+t -1) / t, (ans.1+a-1)/a );
        ans = (o *t,o*a);
    }
    println!("{}",ans.0 + ans.1 );
}