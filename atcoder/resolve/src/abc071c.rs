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
    let n:i64 = read();
    let an = read_vec::<i64>();
    let mut ans:Vec<(_,_)> = an.into_iter().fold(HashMap::new(),|mut map ,x|{
        let y = map.get(&x).map(|i| i+1).unwrap_or(1);
        map.insert(x,y);
        map
    }).into_iter().collect::<Vec<(_,_)>>()
    .into_iter()
    .filter(|x| x.1 >= 2)
    .collect();
    if ans.len() < 2 {
        println!("0");
        return;
    }
    ans.sort_by_key(|x| x.0);
    if ans.iter().enumerate().any(|(i,x)| x.1 >= 4 && i == ans.len()-1 ){
        println!("{}",ans.last().unwrap().0 * ans.last().unwrap().0);
        return;
    }
    println!("{}",ans.last().unwrap().0 * ans[ans.len()-2].0);
}