use std::collections::HashSet;
use std::iter::FromIterator;
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
    let s:String = read();
    let n:usize = read();
    let s = s.chars().collect::<Vec<char>>();
    let mut set = HashSet::new();
    for j in 1..s.len()+1{
        let v:Vec<String> = s.windows(j)
        .map(|x| x.iter().map(|c| *c).collect())
        .collect();
        for k in v {
            set.insert(k);
        }
    }
    let mut ans:Vec<String> = set.into_iter()
        .collect();
    ans.sort();
    println!("{}",ans[n-1] );
}