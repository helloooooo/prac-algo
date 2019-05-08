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
    let mut an: Vec<i32> = Vec::new();
    for _ in 0..n {
        an.push(read::<i32>());
    }
    an.sort();
    let mut i = 0;
    let mut ans = 0;
    while i < n {
        let j = an[i as usize];
        let mut count = 0;
        while i < n && an[i as usize] == j {
            count += 1;
            i += 1;
        }
        ans += count % 2;
    }
    println!("{}", ans);
}
