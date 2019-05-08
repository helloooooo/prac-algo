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
    let i: Vec<i32> = read_vec();
    let n = i[0];
    let a = i[1];
    let b = i[2];
    let ans: i32 = (1..(n + 1))
        .map(|x| {
            (
                x,
                x.to_string()
                    .chars()
                    .map(|y| y.to_digit(10).unwrap() as i32)
                    .fold(0, |sum, i| sum + i),
            )
        })
        .filter(|&(_, x)| a <= x && x <= b)
        .map(|(x, _)| x)
        .fold(0, |sum, i| sum + i);
    println!("{:?}", ans);
}
