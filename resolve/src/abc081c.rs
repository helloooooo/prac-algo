use std::collections::HashMap;
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
    let  (n,k) = {
        let t = read_vec::<i64>();
        (t[0],t[1])
    };
    let mut an:Vec<(_,_)> = read_vec::<i64>().into_iter().fold(HashMap::new(),|mut map,x|{
        let y = map.get(&x).map(|i| i+1).unwrap_or(1);
        map.insert(x,y);
        map
    }).into_iter().collect();
    an.sort_by_key(|v|v.1);
    let len = an.len() as i64;
    let ans =if len < k {
        0
    } else{
        an.into_iter().take((len -k) as usize).fold(0,|y,x| y + x.1)
    };
    println!("{}",ans);
}