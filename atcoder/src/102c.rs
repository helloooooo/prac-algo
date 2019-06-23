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
    let n = read::<i32>();
    let mut v = read_vec::<i32>();
    let mut sub = v.clone();
    sub.sort();
    sub.dedup();
    let div = v.iter().enumerate().map(|x| (x.1 - (x.0 as i32)-1)).collect::<Vec<i32>>();
    let hash = div.iter().fold(HashMap::new(),|mut map,x| {
        let y  =map.get(&x).map(|i| i+1).unwrap_or(1);
        map.insert(x,y);
        map
    });
    let next = hash.clone();
    let checker = hash.iter().fold(0,|ans,(x,y)| *y);
    let b = if hash.into_iter().all(|(x,y)| y == checker){
        next.into_iter().fold(100000000000000,|ans,(x,y)| if ans > (*x).abs(){
            (*x).abs()
        }else {
            ans
        })
    } else{
        next.into_iter()
        .fold((0,0),|ans,(x,y)| if y > ans.1{
                (*x,y)
            }else {
                ans
        }).0
    };
    let ans = v.iter().enumerate().fold(0,|x,y| (x + (y.1 - (y.0 as i32)-1 - b).abs()));
    println!("{:?}",ans);
}