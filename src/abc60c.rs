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

fn main() {
    let nt:Vec<i64> = read_vec();
    let tn:Vec<i64> = read_vec();
    let ans = (0..nt[0]-1).fold(nt[1],|x,y| {
        let sub = (tn[(y+1) as usize]- tn[y as usize]).abs();
        x + min(nt[1],sub)
    });
    println!("{}",ans);
}