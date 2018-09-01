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
    let nk = read_vec::<i64>();
    let an = read_vec::<i64>();
    let mut sub = 0;
    let mut sv:Vec<i64> = Vec::new();
    for j in 0..nk[0] {
        sub += an[j as usize];
        sv.push(sub);
    }
    let mut ans = 0;
    for j in 0..(nk[0]-nk[1]+1){
       ans += sv[(j+nk[1]-1) as usize] - sv.get((j-1) as usize).unwrap_or(&0);
    }
    println!("{}", ans);
}