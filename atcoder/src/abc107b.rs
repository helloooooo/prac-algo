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
    let hw = read_vec::<i64>();
    let mut f: Vec<Vec<char>> = Vec::new();
    for _ in 0..hw[0] {
        f.push(read::<String>().chars().collect());
    }
    // let mut ans = Vec::new();
    for j in 0..hw[0] {
        if f[j as usize].iter().all(|&x| x == '.') {
            continue;
        }
        for k in 0..hw[1] {
            if f[j as usize][k as usize] == '.' {
                for l in 0..hw[0] {
                    if f[l as usize][k as usize] == '#' {
                        print!(".");
                        break;
                    }
                }
            } else {
                print!("#");
            }
        }
        println!("");
    }
}
