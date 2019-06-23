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
    let s:Vec<i64> = read::<String>().chars()
        .map(|x| x.to_digit(10).unwrap() as i64)
        .collect();
    let k:usize = read();
    let target:Vec<i64> = s.into_iter().take(k).collect();
    let ans = if target.iter().all(|&x| x == 1) {
        1
    } else {
        *target.iter().filter(|&x| *x != 1).nth(0).unwrap()
    };
    println!("{}",ans );
}