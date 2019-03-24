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
    (0..n).map(|_| read
fn main(){
    let mut s:Vec<char> = read::<String>().chars().collect();
    let mut t:Vec<char>  = read::<String>().chars().collect();
    let s_map = s.iter().fold(HashMap::new(),|mut map,c|{
        let y = map.get(&c).map(|i| i + 1).unwrap_or(1);
        map.insert(c, y);
        map
    });
    let t_map = t.iter().fold(HashMap::new(),|mut map,c|{
        let y = map.get(&c).map(|i| i + 1).unwrap_or(1);
        map.insert(c, y);
        map
    }).collect::<Vec<(char,i64)>>();
    let 
    // println!("{}",s.len() );
    // println!("{}",t.len() );
    // println!("{}",if s.len()-t.len() == 0 {"Yes"} else {"No"} );
}