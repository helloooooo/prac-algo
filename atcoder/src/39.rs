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
    let input = read_vec::<f32>();
    let n = read::<i32>();
    let mut v = Vec::new();
    let dist = input[4] * input[5];
    for _ in 0..n{
        let xy = read_vec::<f32>();
        v.push((xy[0],xy[1]));
    }
    let ans =if v.iter().filter(|point| 
        ((point.0 - input[0]).powf(2.0) + (point.1 - input[1]).powf(2.0)).sqrt() + ( (input[2] - point.0).powf(2.0) + (input[3] - point.1 ).powf(2.0)).sqrt() <= dist  ).count() > 0 {
            "YES".to_string()
        } else {
            "NO".to_string()
        };
    println!("{}",ans);
}