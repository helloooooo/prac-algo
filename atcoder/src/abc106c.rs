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
    let s: String = read();
    let k: i64 = read();
    let s: Vec<char> = s.chars().collect();
    if s.len() == 1 {
        println!("{}", s[0]);
        return;
    } else if k <= s.len() as i64 {
        if s.iter().take(k as usize).all(|&x| x == '1') {
            println!("1");
            return;
        } else {
            let ans = s.into_iter().filter(|&x| x != '1').nth(0).unwrap_or('1');
            println!("{}", ans);
            return;
        }
    } else {
        let ans = s.into_iter().filter(|&x| x != '1').nth(0).unwrap_or('1');
        println!("{}", ans);
        return;
    }
}
