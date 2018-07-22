use std::collections::HashSet;
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
    let n : u64 = read();
    let v = read_vec::<u64>();
    let u = v.iter().filter(|&x| x < &3200).map(|x| x/400).collect::<HashSet<u64>>().len();
    let o = v.iter().filter(|&x|x >= &3200).count();
    let ans_u = if u == 0{
        1
    } else {
        u
    };
    let ans_o = u + o;
    println!("{} {}",ans_u, ans_o );
}