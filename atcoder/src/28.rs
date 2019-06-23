
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
    let x = read::<i64>();
    let mut v:Vec<i64> = Vec::new();
    for i in 1..1001{
        for j in 2..1001{
            if (i as i64).pow(j)  > x.into() {break;}
            v.push((i as i64).pow(j));
        }
    }
    println!("{}",v.iter().max().unwrap());
}