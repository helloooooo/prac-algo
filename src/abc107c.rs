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
fn main(){
    let mut nk = read_vec::<i64>();
    let mut v = read_vec::<i64>();
    // v.push(0);
    v.sort();
    v.dedup();
    let t = v.clone();
    let ans = solve(0, t, nk[1], 0);
    println!("{}",ans );
}
fn solve(ans:i64,v:Vec<i64>,k:i64,p:i64)->i64{
    if k == 0{
        ans
    }else if v.len() == 1{
        v[0]
    } else {
        let mut next = v.iter().enumerate().map(|x| (x.0,(x.1-p).abs())).collect::< Vec< (usize,i64)>>();
        next.sort_by_key(|k| k.1);
        let p1 =  v[next[0].0 as usize];
        let p2 =  v[next[1].0 as usize];
        let sub1 = solve(ans+next[0].1, v.iter().filter(|x| *x != &p1).collect::<Vec<i64>>(), k-1,p1);
        let sub2 = solve(ans+next[1].1, v.clone().into_iter().filter(|&x| x != p2).collect::<Vec<i64>>(), k-1, p2);
        min(sub1, sub2)
    }
}