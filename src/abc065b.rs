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
    let n = read::<u64>();
    let mut v = Vec::new();
    for _ in 0..n{
        v.push(read::<u64>()-1);
    }
    let mut m = 0;
    for i in 0..n {
        m = v[m as usize];
        if m == 1 {
            println!("{}",i+1);
            return;
        }
    }
    println!("-1");
}