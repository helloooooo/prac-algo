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

// struct Points {
//     start:(i64,i64),
//     goal: (i64,i64),
//     n : i64,
//     m : i64,

// }
fn main(){
    let wh = read_vec::<i64>();
    let (w,h) = (wh[0],wh[1]);
    let ans = (1..w+h-2).fold(1,|y,x| {
        (y % 1000000007) * (x % 1000000007)
    });
    println!("{}",ans );
}