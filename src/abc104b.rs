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
    let s: String = read();
    let len = s.chars().count();
    if s.chars()
        .skip(2)
        .take(len - 3)
        .filter(|&x| x == 'C')
        .count()
        == 1
    {
        if s.chars().nth(0).unwrap() == 'A' {
            if s.chars().filter(|&x| x.is_lowercase()).count() == (len - 2) {
                println!("AC");
                return;
            }
        }
    }
    println!("WA");
}
//s.chars().skip(2).take(len - 1).filter(|&x| x == 'C').count() == 1
