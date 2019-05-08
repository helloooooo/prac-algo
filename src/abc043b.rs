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
    let s: Vec<char> = read::<String>().chars().collect();
    let mut ans = Vec::new();
    for x in &s {
        match *x {
            '0' => ans.push('0'),
            '1' => ans.push('1'),
            'B' => {
                ans.pop();
            }
            _ => unimplemented!(),
        }
    }
    for x in &ans {
        print!("{}", x);
    }
}
