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
    let n: u32 = read();
    let data: Vec<Vec<i32>> = read_vec2(n - 1);
    for i in 0..n {
        let mut time = 0;
        for j in i..(n - 1) {
            if time < data[j as usize][1] {
                time = data[j as usize][1];
            } else if time % data[j as usize][2] == 0 {
            } else {
                time = time + data[j as usize][2] - (time % data[j as usize][2]);
            }
            time += data[j as usize][0];
        }
        println!("{}", time);
    }
}
