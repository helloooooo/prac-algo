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
    let n = read::<i64>();
    let mut ans: Vec<Vec<char>> = vec![vec![' '; n as usize]; n as usize];
    let sn: Vec<Vec<char>> = read_vec2::<String>(n as u32)
        .into_iter()
        .map(|x| x[0].chars().collect())
        .collect();
    for j in 0..n {
        for k in 0..n {
            ans[k as usize][(n - j - 1) as usize] = sn[j as usize][k as usize];
        }
    }
    for j in 0..n {
        for k in 0..n {
            print!("{}", ans[j as usize][k as usize]);
        }
        println!("");
    }
}
