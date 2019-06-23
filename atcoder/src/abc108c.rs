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
    let (n, k) = {
        let r = read_vec::<i64>();
        (r[0], r[1])
    };
    if k % 2 != 0 {
        println!("{}", (n / k).pow(3));
    } else {
        let mut c: i64 = 0;
        for i in 0..n + 1 {
            if i % k == k / 2 {
                c += 1;
            }
        }
        println!("{}", (n / k).pow(3) + c.pow(3));
    }
}
