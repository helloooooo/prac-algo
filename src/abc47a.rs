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
pub const M: i32 = 1000000007;
fn main() {
    let ans = solve();
    println!("{}",ans );
}
fn solve() -> String{
    let mut abc = read_vec::<i32>();
    abc.sort();
    check(abc[0] + abc[1],abc[2])
}
fn check(x:i32,y:i32) -> String{
        let ans =  if x == y{
            "Yes".to_string()
        } else{
            "No".to_string()
        };
        ans
}
