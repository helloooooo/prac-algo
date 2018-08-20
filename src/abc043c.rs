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
    let n : i64 = read();
    let v  = read_vec::<i64>();
    let sum = v.iter().fold(0,|y,x| y + x );
    let len = v.len() as i64;
    let ave = if sum % len == 0 {
        sum / len
    } else {
        (sum + sum % len ) / len
    };
    let ans = v.iter().map(|x| (x - ave) * (x - ave))
        .fold(0 , |y ,x | y + x );
    println!("{}",ans );

}