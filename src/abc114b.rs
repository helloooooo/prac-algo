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
    let s:Vec<i64> = read::<String>().chars().map(|x| x.to_digit(10).unwrap() as i64).collect();
    let mut ans = 1e10 as i64;
    for i in 0..(s.len() -2){
        let v = s[i]*100 + s[(i + 1) as usize]* 10 + s[(i+2) as usize];
        let dif = (v- 753).abs();
        ans = min(ans,dif);
    }
    println!("{}",ans);
}