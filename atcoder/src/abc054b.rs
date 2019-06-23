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
    let nm = read_vec::<u64>();
    let (n, m) = (nm[0], nm[1]);
    let mut an = Vec::new();
    let mut bn = Vec::new();
    for _ in 0..n {
        an.push(read::<String>());
    }
    for _ in 0..m {
        bn.push(read::<String>());
    }
    // let an = read_vec2::<String>(n as u32);
    // let bn = read_vec2::<String>(m as u32);
    let an = an
        .into_iter()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let bn = bn
        .into_iter()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    for j in 0..(n - m + 1) {
        for k in 0..(n - m + 1) {
            let mut flag = true;
            for x in 0..m {
                for y in 0..m {
                    if an[(j + x) as usize][(k + y) as usize] != bn[x as usize][y as usize] {
                        flag = false
                    }
                }
            }
            if flag {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
