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
    let nk:Vec<i64> = read_vec();
    let mut v :Vec<(i64,i64)> = Vec::new();
    for _ in 0..nk[0] {
        let r:Vec<i64> = read_vec();
        v.push((r[0],r[1]));
    }
    v.sort();
    let mut key = nk[1];
    let mut ans = 0;
    for (a,b) in v{
        key -= b;
        if key <= 0{
           ans =  a;
            break;
        }
    };
    println!("{}",ans);
}