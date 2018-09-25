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
    let (n,m,x,y) = {
        let t = read_vec::<i64>();
        (t[0],t[1],t[2],t[3])
    };
    let mut xn = read_vec::<i64>();
    xn.sort();
    let mut yn = read_vec::<i64>();
    yn.sort();
    let x_end = xn[n as usize -1];
    let y_start = yn[0];
    if x_end < y_start {
        let diff = y_start - x_end;
        let ans = (1..diff+1).map(|s| x_end + s )
            .filter(|&s| x < s  && s <= y)
            .count();
        if ans != 0{
            println!("No War");
            return;
        }
    }
    println!("War");
}
