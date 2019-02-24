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
    let st = read::<String>();
    let s:Vec<_> = st.clone().split('/').collect();
    let year:i32 = s[0].parse::<i32>().unwrap();
    let month:i32= s[1].parse::<i32>().unwrap();
    let day:i32 = s[2].parse::<i32>().unwrap();
    let ans = if year <= 2018 {
        "Heisei"
    } else if year >= 2019 && month <= 4{
        "Heisei"
    } else {
        "TBD"
    };
    println!("{}",ans);
}