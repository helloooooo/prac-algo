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
    let i1: Vec<u32> = read_vec();
    let mut i2: Vec<String> = Vec::new(); // parse to char from string
    for i in 0..i1[0] as i32 {
        let sub_str: String = read();
        i2.push(sub_str);
    }
    let v = vec![-1, 0, 1];
    for i in 0..i1[0] as i32 {
        for j in 0..i1[1] as i32 {
            let mut ans = 0;
            for k in 0..3 {
                for l in 0..3 {
                    let x = j + v[l];
                    let y = i + v[k];
                    if 0 <= y && y < i1[0] as i32 && 0 <= x && x < i1[1] as i32 {
                        if i2[y as usize].chars().nth(x as usize).unwrap() == '#' {
                            ans += 1;
                        }
                    }
                }
            }
            if i2[i as usize].chars().nth(j as usize).unwrap() == '#' {
                print!("#");
            } else {
                print!("{}", ans);
            }
        }
        println!("");
    }
}
