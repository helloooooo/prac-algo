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
    let nq = read_vec::<i64>();
    let mut v = vec![0; nq[0] as usize];
    let lrt = read_vec2(nq[1] as u32);
    for t in &lrt {
        for j in t[0] - 1..t[1] {
            v[j as usize] = t[2];
        }
    }
    for x in &v {
        println!("{}", x);
    }
}
