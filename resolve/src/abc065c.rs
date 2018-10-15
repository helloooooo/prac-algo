const N: i64 = 1000000007;
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
    let (n,m) = {
        let t = read_vec::<i64>();
        (t[0],t[1])
    };
    let ans = if (n-m).abs() >= 2{
        0
    } else if (n-m).abs() == 1{
       fact(n) * fact(m) 
    } else {
        fact(n) * fact(m)  * 2
    };
    println!("{}",ans % (1e9 as i64 + 7));
}
fn fact(x:i64) -> i64{
    (1..x+1).fold(1,|x,y| (x * y) % (1e9 as i64+7))
}