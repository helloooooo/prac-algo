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
    let i: Vec<i64> = read_vec();
    let x = i[0];
    let y = i[1];
    let ans = run(x, y, 0);
    println!("{}", ans);
}

fn run(x: i64, y: i64, count: i64) -> i64 {
    if x > y {
        count
    } else {
        println!("{}", x);
        run(x * 2, y, count + 1)
    }
}
