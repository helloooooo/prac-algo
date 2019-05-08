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
    let i: Vec<Vec<i32>> = read_vec2(3);
    for i1 in 0..i.len() {
        for i2 in 0..i.len() {
            for j1 in 0..i.len() {
                for j2 in 0..i.len() {
                    if i[i1][j1] + i[i2][j2] != i[i1][j2] + i[i2][j1] {
                        println!("No");
                        return;
                    }
                }
            }
        }
    }
    println!("Yes");
}
