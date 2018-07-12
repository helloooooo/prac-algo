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
    let n = input[0];
    let x = input[1];
    let mut mn = Vec::new();
    for _ in 0..n {
        mn.push(read::<i32>());
    }
    let mini = mn.iter().min().unwrap();
    let sum :i32 = mn.iter().sum();
    let mut ans :i32 = if x - sum > 0{
        (x - sum ) / mini
    } else {
        0
    };
    ans +=n;
    println!("{}",ans);
}