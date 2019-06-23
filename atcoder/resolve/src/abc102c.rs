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
    let mut diff = an.into_iter().enumerate()
        .map(|(j,v)| v- (j+1)as i64)
        .collect::<Vec<i64>>();
    diff.sort();
    let mid = diff[(n/2) as usize];
    let ans = diff.iter().fold(0,|y,x| y + (x-mid).abs());
    println!("{}",ans );    
}