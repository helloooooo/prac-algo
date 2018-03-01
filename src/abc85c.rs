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
    let ny:Vec<i32> = read_vec();
    let (n,y) = (ny[0],ny[1]);
    for i in 0..n+1 {
        for j in  0..n+1 -i  {
            let z = n - i - j;
            if i * 10000 + j * 5000 + 1000 * z == y {
                println!("{} {} {}",i,j,z);
                return;
            }
        }
    }
    println!("-1 -1 -1");
}