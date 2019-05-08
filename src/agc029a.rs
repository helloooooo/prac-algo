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
    let v: Vec<char> = read::<String>().chars().collect();
    let mut sub = vec![];
    let mut count: i64 = 0;
    for c in &v {
        if *c == 'B' {
            count += 1;
        }
        sub.push(count);
    }
    let ans = v.into_iter().enumerate().fold(0, |y, x| {
        if x.1 == 'W' {
            let t = y + sub[x.0 as usize];
            t
        } else {
            y
        }
    });
    println!("{:?}", ans);
}
