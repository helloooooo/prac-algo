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
    let n = read::<i64>();
    let mut v = vec![];
    for _ in 0..n{
        v.push(read::<i64>());
    }
    v.sort();
    let sum:i64 = v.iter().sum();
    let sub = v.into_iter().filter(|x| x % 10 != 0).collect::<Vec<_>>();
    let ans = if sum % 10 == 0 {
        if sub.len() == 0 {
            0
        } else {
            sum - sub[0]
        }
    } else {
        sum
    };
    println!("{}",ans);
}