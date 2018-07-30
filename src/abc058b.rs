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
    let o : String= read();
    let e : String = read();
    let e = if e.len() < o.len(){
        e + " "
    } else {
        e
    };
    let mut ans : String = o.chars().zip(e.chars())
        .flat_map(|(x,y)| vec![x,y]).collect();
    println!("{}",ans );
}