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
    let an = read_vec::<usize>();
    let mut ans:Vec<(_,_)> = an.into_iter().enumerate().collect();
    ans.sort_by_key(|&(a,b)| b);
    for &(k,_) in &ans.into_iter().rev().collect::<Vec<(_,_)>>() {
        println!("{}",k+1 );
    }
}