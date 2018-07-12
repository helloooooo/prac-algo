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
    let n = read::<i32>();
    let tn = read_vec::<i32>();
    let m = read::<i32>();
    let mut pxn = Vec::new();

    for _ in 0..m {
        pxn.push(read_vec::<i32>());
    }

    let sum:i32 = tn.iter().sum();
    let ans:Vec<i32> = pxn.iter().map(|x| sum-tn[(x[0]-1) as usize] +x[1]).collect();
    for x in ans{
        println!("{}",x);
    }
}