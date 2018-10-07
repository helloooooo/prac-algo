use std::collections::HashSet;
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
    let n = read::<usize>();
    let an = read_vec2::<i64>(n as u32);
    let (xt,yt,ht) = {
            let sub = an.iter().filter(|x| x[2] >= 1).nth(0).unwrap();
            (sub[0],sub[1],sub[2])
        };
    let mut set = HashSet::new();
    for cx in 0..101{
        for cy in 0..101{
            let h = ht + (cx-xt).abs() + (cy-yt).abs();
            set.insert((cx,cy,h));
        }
    }
    let ans = set.into_iter().collect::<Vec<(_,_,_)>>()
        .into_iter().filter(|a| an.iter().all(|b| {
            let (xb,yb,hb) =(b[0],b[1],b[2]);
            hb == max(a.2-(a.0-xb).abs()-(a.1-yb).abs(),0)
        })).nth(0).unwrap();
    println!("{} {} {}",ans.0,ans.1,ans.2);

}
