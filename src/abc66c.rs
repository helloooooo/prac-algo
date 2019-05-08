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
    let an: Vec<i64> = read_vec();
    let mut a: Vec<i64> = Vec::new(); // % 2 == 0
    let mut b: Vec<i64> = Vec::new(); //  %2  != 0
    for i in 0..an.len() {
        if i % 2 != 0 {
            a.push(an[i as usize]);
        } else {
            b.push(an[i as usize]);
        }
    }
    let ans_n = if n % 2 == 0 {
        a.reverse();
        a.append(&mut b);
        a
    } else {
        b.reverse();
        b.append(&mut a);
        b
    };
    let ans = ans_n
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ")
        .to_string();
    println!("{}", ans);
}
