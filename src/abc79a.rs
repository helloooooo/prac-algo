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
    let n : String = read();
    let v: Vec<i32> =n.chars().map(|x| x as i32).collect();
    if v[0] == v[1] && v[1] == v[2] {
        println!("Yes");
    } else if v[1] == v[2] && v[2] == v[3] {
        println!("Yes");
    } else{
        println!("No");
    }
}   