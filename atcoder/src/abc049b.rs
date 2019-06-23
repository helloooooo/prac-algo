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
    let hw = read_vec::<u64>();
    let mut v: Vec<Vec<char>> = Vec::new();
    for _ in 0..hw[0] {
        v.push(read::<String>().chars().collect());
    }
    for j in 0..hw[0] * 2 {
        for k in 0..hw[1] {
            let i = (j) / 2;
            print!("{}", v[i as usize][k as usize]);
        }
        println!("");
    }
}
