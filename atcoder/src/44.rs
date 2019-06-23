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
    let n = read::<i32>();
    let s = read::<String>();
    let cs = s.chars().collect::<Vec<char>>();
    let mut ans = Vec::new();
    for j in 0..n/2+2{
        let mut sub = cs.clone();
        let mut cut = sub.split_at(j as usize);
        let mut a1 = cut.0.to_vec();
        let mut a2= cut.1.to_vec();
        let sum = a1.iter().filter(|&&x| x == 'W' ).count() + a2.iter().skip(1).filter(|&&x| x == 'E' ).count();
        ans.push(sum);
    }
    println!("{}",ans.iter().min().unwrap());
}
