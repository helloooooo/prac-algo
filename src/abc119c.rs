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
    let (n,a,b,c) = {
        let t = read_vec::<usize>();
        (t[0],t[1],t[2],t[3])
    };
    let mut ln = vec![];
    for _ in 0..n{
        ln.push(read::<usize>());
    }
    ln.sort();
    let mut sub = ln.iter().map(|&x| {
        if x == a  &&  x == b && x == c {
            0
        } else {
            x
        }
    }).filter(|&x| x != 0);
    for i in 0..sub.len() {
        let mut field = vec![vec![0;sub.len()];sub.len()];
        for j in i..sub.len(){
            fieldp[i]p[j] = sub[i] + sub[j];
        }
    }
}