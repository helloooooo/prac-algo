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
    let n: i32 = read();
    let mut sn: Vec<i64> = Vec::new();
    for _ in 0..n {
        sn.push(read::<i64>());
    }
    let sum: i64 = sn.iter().sum();
    let ans = if sum % 10 == 0 {
        let mut sub = sn.iter().filter(|&&x| x % 10 != 0).collect::<Vec<&i64>>();
        sub.sort();
        match sub.len() {
            0 => 0,
            _ => sum - sub[0],
        }
    } else {
        sum
    };
    println!("{}", ans);
}
