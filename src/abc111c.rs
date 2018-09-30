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
    let len = n /2;
    let an = read_vec::<i64>();
    if an.iter().all(|&x| x==an[0]){
        println!("{}",len);
        return;
    }
    let even:Vec<_> = an.iter().enumerate().filter(|&(i,v)| i % 2 == 0).map(|(k,v)|v).collect();
    let odd:Vec<_> =  an.iter().enumerate().filter(|&(i,v)| i % 2 != 0).map(|(k,v)|v).collect();
    let mut even_map:Vec<(_,_)> = even.iter().fold(HashMap::new(), |mut map, x| {
            let y = map.get(&x).map(|i| i + 1).unwrap_or(1);
            map.insert(x, y);
            map
        }).into_iter()
        .collect();
    even_map.sort_by_key(|x| x.1);
    let mut odd_map:Vec<(_,_)> = odd.iter().fold(HashMap::new(), |mut map, x| {
            let y = map.get(&x).map(|i| i + 1).unwrap_or(1);
            map.insert(x, y);
            map
        }).into_iter()
        .collect();
    odd_map.sort_by_key(|x| x.1);
    let even1 = even_map.last().unwrap();
    let odd1 = odd_map.last().unwrap();
    let even2 = if even_map.len() == 1 {
        0
    } else {
        even_map[(even_map.len() -2 )as usize].1
    };
    let odd2 = if odd_map.len() == 1 {
        0
    } else {
        odd_map[(odd_map.len() -2) as usize].1
    };

    let ans = if odd1.0 == even1.0 {
        let first = n - odd1.1 - even2;
        let second = n -even1.1 - odd2;
        std::cmp::min(first,second)
    } else{
        n - odd1.1 - even1.1
    };
    println!("{}",ans);
}