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
    let points = read_vec::<i32>();
    let (sx,sy,tx,ty) = (points[0],points[1],points[2],points[3]);
    let dx = (tx-sx).abs();
    let dy = (ty-sy).abs();
    let mut ans = Vec::new();
    ans.push(create_chars('U', dy));
    ans.push(create_chars('R', dx));
    ans.push(create_chars('D', dy));
    ans.push(create_chars('L', dx));
    ans.push("L".to_string());
    ans.push(create_chars('U', dy+1));
    ans.push(create_chars('R', dx+1));
    ans.push("D".to_string());
    ans.push("R".to_string());
    ans.push(create_chars('D', dy+1));
    ans.push(create_chars('L', dx+1));
    ans.push("U".to_string());
    println!("{}",ans.connect(""));
}
fn create_chars(c:char,time:i32)->String{
    let mut v = Vec::new();
    for _ in 0..time { v.push(c)}
    v.into_iter().collect::<String>()
}