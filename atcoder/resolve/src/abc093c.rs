
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
    let mut an = read_vec::<i64>();
    let mut ans = 0;
    loop {
        an.sort();
        if an[0] == an[1] && an[1] == an[2]{
            println!("{}",ans );
            return;
        }else if an[0] != an[1] && an[1] == an[2]{
            an[0] += 2;
            ans += 1;
        }else if an[0] != an[1] && an[1] != an[2]{
            an[0] += 1;
            an[1] += 1;
            ans += 1;
        }else{
            an[0] += 1;
            an[1] += 1;
            ans += 1;
        }
    }
}