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
    let (h, w) = {
        let t = read_vec::<usize>();
        (t[0], t[1])
    };
    let mut ans = vec![];
    let mut an = read_vec2::<i64>(h as u32);
    for j in 0..h {
        for k in 0..w {
            if an[j][k] % 2 == 1 {
                if k + 1 < w {
                    an[j][k] -= 1;
                    an[j][k + 1] += 1;
                    ans.push((j, k, j, k + 1));
                } else if j + 1 < h {
                    an[j][k] -= 1;
                    an[j + 1][k] += 1;
                    ans.push((j, k, j + 1, k));
                }
            }
        }
    }
    println!("{}", ans.len());
    for (x1, y1, x2, y2) in ans {
        println!("{} {} {} {}", x1 + 1, y1 + 1, x2 + 1, y2 + 1);
    }
}
