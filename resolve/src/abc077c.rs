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
    let n= read::<usize>();
    let mut an = read_vec::<i64>();
    let mut bn = read_vec::<i64>();
    let mut cn = read_vec::<i64>();
    an.sort();
    cn.sort();
    let mut ans = 0;
    for b in &bn {
             let a = an.binary_search(&b);
             let a = match a {
                 Ok(a) => {
                     if a == 0{
                         1
                     } else {
                         a
                     }
                 },
                 Err(a) => {
                     if a == 0 {
                         0
                     } else {
                         a
                     }
                 }
             };
             let c = cn.binary_search(&b);
             let c = match c {
                 Ok(c) => {
                     if c == n-1{
                         1
                    } else {
                        n-(c+1)
                    }
                },
                Err(c) => {
                    if c == n{
                        0
                    } else {
                        n-c
                    }
                }
             };
             ans += a * c;
    }
    println!("{}",ans);
}
