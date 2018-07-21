
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
    let input = read_vec::<i32>();
    let a = input[0];
    let b =input[1];
    let c = input[2];
    let ans  =if (a-c).abs() <= input[3] {
        "Yes".to_string()
    } else if (b- c ).abs() <= input[3] {
        if (a-b).abs() <= input[3]{
        "Yes".to_string()
        } else {
            "No".to_string()
        }
    } else {
        "No".to_string()
    };
    println!("{}",ans);
}