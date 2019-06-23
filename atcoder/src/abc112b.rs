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
    let (n, t) = {
        let t = read_vec::<i64>();
        (t[0], t[1])
    };
    // let mut an = vec![];
    // for _ in 0..n{
    //     an.push(read_vec::<i64>());
    // }
    let an = read_vec2::<i64>(n as u32);
    if an.iter().filter(|x| x[1] <= t).count() == 0 {
        println!("TLE");
    } else {
        let ans = an.iter().filter(|x| x[1] <= t).map(|x| x[0]).min().unwrap();
        println!("{}", ans);
    };
}
