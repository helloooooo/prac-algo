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
fn main(){
    let n:i64 = read();
    let an = read_vec::<i64>();
    let mut under_color = an.iter().map(|x| x / 400 ).filter(|x| *x < 8).collect::<Vec<_>>();
    under_color.sort();
    under_color.dedup();
    let under = under_color.len();
    let all_color = an.iter().filter(|&x| *x >= 3200).count();
    let max = under + all_color;
    let min = if under == 0{
        1
    } else {
        under
    };
    println!("{} {}",min ,max);
}