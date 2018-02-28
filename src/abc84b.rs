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
    let i :Vec<i32> = read_vec();
    let (a,b) = (i[0],i[1]);
    let s_sub :String = read();
    let s:Vec<char> = s_sub.chars().collect();
    let ans = match (0..a).chain(a + 1..(a + b + 1)).all(|x| s[x as usize].is_digit(10)) && s[a as usize] == '-'  {       
        true => "Yes",
        false => "No",
    };
    println!("{}",ans);
}