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
    let an = read_vec::<i64>();
    let ans = std::cmp::min(solve(true, &an),solve(false, &an));
    println!("{}",ans );
    
}
fn solve(mut symbol: bool, v:&Vec<i64>) -> i64{
    let mut sum = 0;
    let mut ans = 0;
    for &a in v{
        sum += a;
        if !symbol && sum >= 0 {
            let next = sum - -1;
            ans += next;
            sum = -1;
        } else if symbol && sum <= 0{
            let next = 1 -sum;
            sum = 1;
            ans += next;
        }
        println!("{}",sum );
        symbol = !symbol;
    }
    ans
}
