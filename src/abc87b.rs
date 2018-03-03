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
    let a :i32 = read(); //500
    let b :i32 = read(); //100
    let c :i32 = read(); //50
    let x :i32 = read(); //合計
    let mut ans = 0;
    for i in 0..a + 1{
        for j in 0..b + 1 {
            for k in 0..c + 1  {
                if i * 500 + j * 100 + k * 50 == x {
                    ans += 1;
                }
            }
        }
    }
    println!("{}",ans);
}