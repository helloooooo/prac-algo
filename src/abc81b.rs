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
    let n: u32 = read();
    let v: Vec<i32> = read_vec();
    let ans = run(v, 0);
    println!("{}", ans);
}
fn run(i: Vec<i32>, count: i32) -> i32 {
    if i.iter().all(|x| x % 2 == 0) {
        run(i.iter().map(|x| x / 2).collect(), count + 1)
    } else {
        count
    }
}
