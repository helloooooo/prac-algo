use std::cmp::min;
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
    let n: i32 = read();
    let mut si: Vec<String> = Vec::new();
    for _ in 0..n {
        si.push(read::<String>());
    }
    let m: i32 = read();
    let mut ti: Vec<String> = Vec::new();
    for _ in 0..m {
        ti.push(read::<String>());
    }

    let mut cp = si.clone();
    cp.dedup();

    let mut ans = -100000000;
    for i in 0..cp.len() {
        let mut plus = 0;
        let mut mi = 0;
        for j in 0..n {
            if si[j as usize] == cp[i as usize] {
                plus += 1;
            }
        }
        for k in 0..m {
            if ti[k as usize] == cp[i as usize] {
                mi += 1;
            }
        }
        if ans < plus - mi {
            ans = plus - mi;
        }
    }
    println!("{}", if ans < 0 { 0 } else { ans });
}
