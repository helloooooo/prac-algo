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
    let (mut a, mut b, k) = {
        let t = read_vec::<usize>();
        (t[0], t[1], t[2])
    };
    for j in 0..k {
        if j % 2 == 0 {
            a = calc(a) / 2;
            b += a;
        } else {
            b = calc(b) / 2;
            a += b;
        }
    }
    println!("{} {}", a, b);
}
fn calc(x: usize) -> usize {
    if x % 2 == 0 {
        x
    } else {
        x - 1
    }
}
