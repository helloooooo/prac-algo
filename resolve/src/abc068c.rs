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
    let (n,m) = {
        let t = read_vec::<i64>();
        (t[0],t[1])
    };
    let an = read_vec2::<i64>(m as u32);
    let bn:Vec<_> = an.iter().filter(|x| x[0] == 1)
            .collect();
    let cn = bn.into_iter().map(|x| x[1]).collect::<Vec<i64>>();
    if an.iter().filter(|x| x[1] == n).map(|x| x[0]).any(|x| cn.contains(&x)) {
    println!("POSSIBLE"); 
    }else {
        println!("IMPOSSIBLE");
    }
}
