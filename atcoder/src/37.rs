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
    let abcd = read_vec::<f32>();
    let ans:String = if abcd[1] / abcd[0] > abcd[3] / abcd[2] {
        "TAKAHASHI".to_string()
    } else if abcd[1] / abcd[0] < abcd[3] / abcd[2] {
        "AOKI".to_string()
    } else{
        "DRAW".to_string()
    };
    println!("{}",ans);
}