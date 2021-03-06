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
    let mut s: Vec<Vec<i32>> = read_vec2(n);
    let mut p: Vec<Vec<i32>> = read_vec2(n);
    // let ans: Vec<(&Vec<i32>,&Vec<i32>)>= s.iter().zip(p.iter()).collect();
    let mut v: Vec<&i32> = Vec::new();
    let mut ans = 0;
    s.iter().zip(p.iter()).map(|x| {
        v.push(
            x.1.split_at((x.0.iter().sum::<i32>() + 1) as usize)
                .0
                .iter()
                .max()
                .unwrap(),
        )
    });

    print!("{:?}", ans);
}
