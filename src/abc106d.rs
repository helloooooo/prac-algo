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
    let nmq = read_vec::<i64>();
    let (n, m, q) = (nmq[0], nmq[1], nmq[2]);
    let mut lr = Vec::new();
    for _ in 0..m {
        let sub = read_vec::<i64>();
        lr.push((sub[0], sub[1]));
    }
    // lr.sort_by_key(|k| k.0);
    lr.reverse();
    let pq = read_vec2::<i64>(q as u32);
    for x in &pq {
        let mut ans = 0;
        for y in &lr {
            if x[0] > y.0 && x[1] < y.1 {
                break;
            } else if x[0] <= y.0 && y.1 <= x[1] {
                ans += 1;
            }
        }
        println!("{}", ans);
    }
}
