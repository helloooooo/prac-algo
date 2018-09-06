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
    let (x,y) =  {
        let t = read_vec::<i64>();
        (t[0],t[1])
    };
    let an = read_vec::<i64>();
    let bn = read_vec::<i64>();
    let mut a_port = false;
    let mut now = an[0]+x;
    let mut ans = 0 ;
    loop {
        if a_port {
            let next = an.clone().into_iter().filter(|&x| x >=now ).nth(0).unwrap_or(0);
            if next == 0 {
                break;
            }
            now = next+x;
            a_port = false;
        } else {
            let next = bn.clone().into_iter().filter(|&x| x >=now ).nth(0).unwrap_or(0);       
            if next == 0 {
                break;
            }
            now = next+y;
            ans += 1;
            a_port = true;
        }
    }
    println!("{}", ans);
}