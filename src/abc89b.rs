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
    let n :i32 = read();
    let mut sn : Vec<String> = read_vec();
    sn.sort();
    sn.dedup();
    let ans = if sn.len() == 4 {
        "Four".to_string()
    }else {
        "Three".to_string()
    };
    println!("{}",ans);
}