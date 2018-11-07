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
    let (n, m) = {
        let t = read_vec::<i64>();
        (t[0], t[1])
    };
    let pmym = read_vec2::<i64>(m as u32);
    let mut test = vec!["".to_string(); m as usize];
    let mut sub: Vec<_> = pmym
        .clone()
        .into_iter()
        .enumerate()
        .map(|x| (x.1, x.0))
        .collect();
    // sub.sort_by_key(|k| k.1[1]);
    sub.sort();
    // println!("{:?}",sub);
    let mut b_key = 0;
    let mut c = 0;
    for (v, k) in sub {
        if b_key != v[0] {
            c = 1;
        } else {
            c += 1;
        }
        test[k] = format!("{:>06}{:>06}", v[0], c);
        b_key = v[0]
    }
    for v in test {
        println!("{}", v);
    }
}
