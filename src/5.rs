
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
    let input = read_vec::<u32>();
    let n = input[0];
    let a = input[1];
    let b = input[2];
    let ans = (1..n+1)
                .filter(|x| {
                    let sum = x.to_string()
                        .chars()
                        .map(|c| (c as u8 - b'0') as u32)
                        .sum::<u32>();
                    a <= sum && sum <=b 
                })
                .sum::<u32>();
    println!("{}",ans);
}