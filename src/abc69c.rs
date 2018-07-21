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
    let n : i32 = read();
    let an : Vec<i32> = read_vec();
    let count_an = an.iter().fold((0,0,0), |(a,b,c),d| {
        if d % 4 == 0 {
            (a+1,b,c)
        } else if d % 2 == 0 {
            (a,b + 1,c)
        }else {
            (a,b,c+1)
        }
    });
    let (a,b) = (count_an.0,count_an.2 + if count_an.1 == 0 { 0 } else {1});
    let  ans = if a  + 1>= b { "Yes" } else { "No" };
    println!("{}",ans);
}