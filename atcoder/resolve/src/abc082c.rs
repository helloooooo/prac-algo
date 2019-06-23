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
    let n = read::<i64>();
    let an = read_vec::<i64>();
    let ans = an.iter().fold(HashMap::new(),|mut map,x|{
        let y = map.get(&x).map(|i| i+1).unwrap_or(1);
        map.insert(x,y);
        map
    }).into_iter().collect::<Vec<(_,_)>>()
    .into_iter()
    .map(|(k,v)|{
        if v  < *k {
            v 
        } else{
            v - k
        }
    })
    .fold(0,|y,x| y + x);
    println!("{}",ans);
}