use std::collections::HashMap;
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
    let n:usize = read();
    let mut v:Vec<Vec<char>> = vec![];
    for _ in 0..n{
        v.push(read::<String>().chars().collect());
    }
    let count_v:Vec<_> = v.into_iter().map(|x| {
        x.into_iter().fold(HashMap::new(),|mut map,x|{
            let y = map.get(&x).map(|i| i+1).unwrap_or(1);
            map.insert(x,y);
            map
        })
    }).collect();
    let char_v = (b'a'..b'z'+1).map(|x| x as char).collect::<Vec<char>>();
    let mut ans= char_v.into_iter()
    .fold(HashMap::new(),|mut map,x|{
        let min =  count_v.iter()
        .fold(1e10 as i64,|a,b|{
            let v: i64 = b.get(&x).map(|i| *i).unwrap_or(0);
            min(a,v)
        });
        map.insert(x,min);
        map
    }).into_iter().collect::<Vec<_>>();
    ans.sort();
    let ans:String = ans
    .into_iter()
    .flat_map(|(c,x)| vec![c;x as usize])
    .collect();
    println!("{}",ans );
}