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
    let s = read::<String>();
    let w = read::<i64>();
    let cs = s.chars().collect::<Vec<char>>();
    let first = cs[0];
    let mut ans = cs.into_iter().skip(1).enumerate().filter(|x| (x.0 as i64+1) % w == 0 ).map(|x|x.1).collect::<Vec<char>>();
    ans.insert(0,first);
    for c in ans{
        print!("{}",c);
    }
}