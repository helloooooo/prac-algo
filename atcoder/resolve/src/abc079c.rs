use std::collections::HashMap;
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
    let (a,b,c,d) = {
        let t:Vec<i64> =read::<String>().chars().map(|x|  x.to_digit(10).unwrap() as i64).collect();
        (t[0],t[1],t[2],t[3])
    };
    if a + b + c + d == 7 { println!("{}+{}+{}+{}=7",a,b,c,d); return;}
    if a + b + c - d == 7 { println!("{}+{}+{}-{}=7",a,b,c,d); return;}
    if a + b - c + d == 7 { println!("{}+{}-{}+{}=7",a,b,c,d); return;}
    if a + b - c - d == 7 { println!("{}+{}-{}-{}=7",a,b,c,d); return;}
    if a - b - c - d == 7 { println!("{}-{}-{}-{}=7",a,b,c,d); return;}
    if a - b - c + d == 7 { println!("{}-{}-{}+{}=7",a,b,c,d); return;}
    if a - b + c - d == 7 { println!("{}-{}+{}-{}=7",a,b,c,d); return;}
    if a - b + c + d == 7 { println!("{}-{}+{}+{}=7",a,b,c,d); return;}
}