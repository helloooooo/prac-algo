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
    let s_sub : String = read();
    let mut s:Vec<char> = s_sub.chars().collect();
    s.sort();
    s.dedup();

    let a = 'a' as u8;
    let mut alphas:Vec<char> = (a..a+26).map(|x| x as char).collect();
    for i in 0..26 {
        if !s.contains(&alphas[i]){
            return println!("{}",alphas[i]);
        }
    }
    println!("None");
}