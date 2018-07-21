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
    let patterns : Vec<String> = ["dream","dreamer","erase","eraser"]
    .iter()
    .map(|x| x.chars().rev().collect())
    .collect();
    let s :String = read::<String>().chars().rev().collect();
    let mut s = &s[..];
    let mut succeded = true;
    while s.len() > 0 {
        let matched = patterns.iter().find(|&p| s.find(p) == Some(0));
        if let Some(p) = matched {
            s  = &s[p.len()..];
        } else {
            succeded = false;
            break;
        }
        println!("{}",s);
    }
    
}