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
    let mut sa: Vec<char> = read::<String>().chars().collect();
    let mut sb: Vec<char> = read::<String>().chars().collect();
    let mut sc: Vec<char> = read::<String>().chars().collect();
    sa.reverse();
    sb.reverse();
    sc.reverse();
    let mut p = 'a';
    let mut ans = ' ';
    loop {
        if p == 'E' {
            break;
        }
        match p {
            'a' => {
                p = sa.pop().unwrap_or('E');
                ans = 'A';
            }
            'b' => {
                p = sb.pop().unwrap_or('E');
                ans = 'B';
            }
            'c' => {
                p = sc.pop().unwrap_or('E');
                ans = 'C';
            }
            _ => unimplemented!(),
        }
    }
    println!("{}", ans);
}
