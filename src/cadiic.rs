use std::collections::HashMap;
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
    let (n,p) = {
        let t = read_vec::<i64>();
        (t[0],t[1])
    };
    let ans:i64 = if n == 1 {
        p
    } else {
        let res = trivial_division(p as u64);
        res.into_iter().fold(HashMap::new(),|mut map,x| {
            let y = map.get(&x).map(|t| t+1).unwrap_or(1);
            map.insert(x,y);
            map
            })
            .into_iter()
            .filter(|&(x,y)|{
                y >= n
            }).fold(1,|y,(i,v)|{
                y *  i.pow((v/n) as u32) as i64
            })
    };
    println!("{}",ans);

}

fn trivial_division(n: u64) -> Vec<u64> {
    let mut result = n;
    for i in 2..((n as f64).sqrt() as u64)+1 {
        if n % i == 0 {
            result = i;
            break;
        }
    }

    if result == n {
        return vec![n];
    } else {
        let mut v1 = vec![result];
        let mut v2 = trivial_division(n / result);

        v1.append(&mut v2);

        return v1;
    }
}