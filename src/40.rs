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
    let line = read_vec::<i32>();
    let (ax,ay,bx,by) = (line[0],line[1],line[2],line[3])
    let n = read::<i32>();
    let mut v = Vec::new();
    let mut sub = 0;
    for _ in 0..n{
        let xy = read_vec::<i32>();
        v.push((xy[0],xy[1]));
    }
    for j in 0..n{
        let k = j + 1;
        sub += if  crossline(v[j].0, v[j].1,v[k].0, v[k].1, ax,ay,bx,by) { 1 } else {0} ;
    }
}
fn crossline(x1:i32,y1)