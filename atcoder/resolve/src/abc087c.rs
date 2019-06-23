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
    let n:usize = read();
    let an = read_vec::<i64>();
    let bn = read_vec::<i64>();
    let mut ans = 0 as i64;
    for j in 0..n { 
        let now_point = an.iter().take(j+1).fold(0,|y,x| y + x) + bn.iter().skip(j).fold(0,|y,x| y + x);
        ans = std::cmp::max(ans,now_point);
    }
    println!("{}",ans);
}
