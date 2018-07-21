
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
    let a:u32 = read();
    let b:u32 = read();
    let c:u32 = read();
    let x:u32 = read();
    let mut ans = 0;
    for i in 0..a + 1{
        for j in 0..b + 1{
            for k in 0..c +1{
                if i * 500 + 100 * j + 50 * k == x {
                    ans += 1;
                }
            }
        }
    }
    println!("{}",ans);
}