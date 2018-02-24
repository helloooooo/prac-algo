use std::collections::HashMap;
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
    let nk: Vec<i32> = read_vec();
    let an: Vec<i32> = read_vec();
    let mut m: HashMap<i32, i32> = HashMap::new();
    for x in an {
        let ins_value = match m.get(&x) {
            Some(y) => y + 1,
            None => 1,
        };
        m.insert(x, ins_value);
    }
    let mut v: Vec<(&i32, &i32)> = m.iter().collect();

    v.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
    let ans = run(&mut v, nk[1], 0);
    println!("{}", ans);
}
fn run(v: &mut Vec<(&i32, &i32)>, k: i32, count: i32) -> i32 {
    if v.len() > k as usize {
        let (_, x) = v.pop().unwrap();
        run(v, k, count + x)
    } else {
        count
    }
}
