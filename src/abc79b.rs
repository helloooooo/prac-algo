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
    let n : i64 = read();
    match n {
        0 =>{
            println!("2");
        },
        1 => {
            println!("1");
        }
        _ => {
            let ans = calc(n);
            println!("{}",ans);
        }
    }

}       
fn calc(n: i64) -> i64 {
    let mut f1 = 2;
    let mut f2 = 1;
    let mut tmp = 0;
    for _ in 0..n - 1 {
        tmp = f1 + f2;
        f1 = f2;
        f2 = tmp;
    }
    tmp
}