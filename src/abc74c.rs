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
    let input: Vec<i32> = read_vec();
    let a = input[0];
    let b = input[1];
    let c = input[2];
    let d = input[3];
    let e = input[4];
    let f = input[5];
    let mut x_v = calc_x(&a, &b, &f);
    let mut x_y = calc_y(&c, &d, &f);
    x_v.sort();
    x_v.dedup();
    x_y.sort();
    x_y.dedup();
    let ans_v = create_pair(&x_v, &x_y);
    let ans = ans_v
        .iter()
        .filter(|x| x.0 != 0 && x.0 + x.1 < f && 100 * x.1 <= x.0 * e)
        .map(|x| (x.0 + x.1, x.1))
        .max_by_key(|x| 100 * x.1 / x.0)
        .unwrap();
    println!("{} {}", ans.0, ans.1);
}

fn calc_x(a: &i32, b: &i32, f: &i32) -> Vec<i32> {
    // calc water weight
    let mut v: Vec<i32> = Vec::new();
    for i in 0.. {
        let x = (100 * a * i);
        for j in 0.. {
            let y = 100 * b * j;
            if x + y > *f {
                break;
            }
            v.push(x + y);
        }
        if x >= *f {
            break;
        }
        v.push(x);
    }
    v
}

fn calc_y(c: &i32, d: &i32, f: &i32) -> Vec<i32> {
    // calc water weight
    let mut v: Vec<i32> = Vec::new();
    for i in 0.. {
        let x = c * i;
        for j in 0.. {
            let y = d * j;
            if (x + y) > *f {
                break;
            }
            v.push(x + y);
        }
        if x >= *f {
            break;
        }
        v.push(x);
    }
    v
}

fn create_pair(n: &Vec<i32>, m: &Vec<i32>) -> Vec<(i32, i32)> {
    let mut v: Vec<(i32, i32)> = Vec::new();
    for i in 0..n.len() {
        for j in 0..m.len() {
            v.push((n[i as usize], m[j as usize]));
        }
    }
    v
}
