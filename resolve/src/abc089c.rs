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
    let cs = vec!['M','A','R','C','H'];
    let mut map = HashMap::new();
    for _ in 0..n{
        *map.entry(read::<String>().chars().nth(0).unwrap()).or_insert(0) += 1;
    }
    let master:Vec<i64> = cs.iter().map(|&c| *map.entry(c).or_insert(0)).collect();
    let f = vec![0,0,0,0,0,0,1,1,1,2];
    let s = vec![1,1,1,2,2,3,2,2,3,3];
    let t = vec![2,3,4,3,4,4,3,4,4,4];
    let mut ans = 0;
    for j in 0..10 as usize{
        ans += master[f[j]] * master[s[j]] * master[t[j]];
    }
    println!("{}",ans );
}