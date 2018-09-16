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
    let res = an.into_iter().fold(0,|ans,x| ans + solve(x, 0));
    println!("{}",res );
}

fn solve(x:i64,ans:i64)-> i64{
    if x % 2 != 0{
        ans
    } else {
        solve(x/2, ans+1)
    }
}