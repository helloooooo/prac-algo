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
    let nm = read_vec::<i64>();
    let (n,m) = (nm[0],nm[1]);
    let abn:Vec<Vec<i64>> = read_vec2(n as u32);
    let cdn:Vec<Vec<i64>> = read_vec2(m as u32);
    let ans = abn.iter().fold(Vec::new(),|mut y,x| {
        let mut calc = cdn.iter().enumerate().map(|(i,cd)| (i,(x[0] - cd[0]).abs() + (x[1]  + cd[1]).abs())).collect::<Vec<(usize,i64)>>();
        calc.sort_by_key(|k| k.1);
        calc.iter().for_each(|k| println!("{}:{}",k.0,k.1));
        y.push(calc[0].0+1);
        y
    });
    for i in &ans{
        println!("{}",i);
    }

} 