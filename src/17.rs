use std::cmp::min;
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
    let n = read::<i64>();
    let mut xn = read_vec::<i64>();
    xn.sort();
    // if xn.contains(&0){
    //       xn.retain(|&x| x== 0);
    // }
  
    let mut ans = (xn[1],xn[0]);
    let mut v:Vec<i64> = Vec::new();
    let maxv = xn.iter().max().unwrap();
    let sub = xn.iter().filter(|&x| x != maxv).max_by_key(|&&x| min(x,(maxv -x))).unwrap();
    let ans = (maxv,sub);

    println!("{} {}",ans.0,ans.1 );
}
fn comb(a:i64, b:i64) -> i64 {
    let x = (a-b+1..a + 1).fold(1,|n,m| n * m);
  
    let y =(1..b+1).fold(1,|n,m| n * m);
  
    x / y
}