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
    let mut an = read_vec::<i64>();
    an.sort();
    an.reverse();
    let a = an.iter()
        .enumerate()
        .filter(|&(k,v)| k % 2 == 0)
        .map(|(k,v)|v)
        .fold(0,|y,x| y+x);
    let b = an.iter()
        .enumerate()
        .filter(|&(k,v)| k % 2 == 1)
        .map(|(k,v)|v)
        .fold(0,|y,x| y+x);
    println!("{}", a -b );
} 