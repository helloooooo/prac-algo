
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
    let n = read::<i64>();
    let mut xn = read_vec::<i64>();
    let sub = xn.clone();

    xn.sort();
    let num = ((n - 2 ) /2) as usize;
    let f_ans = xn[num];
    let a_ans = xn[num+1];
    for i in 0..n{
        if sub[i as usize] <= f_ans {
            println!("{}",a_ans);
        } else {
             println!("{}",f_ans);
        }
    }
}