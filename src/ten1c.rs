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
    let n: usize = read();
    let mut an = vec![];
    for _ in 0..n {
        an.push(read::<i64>());
    }
    an.sort();
    let mut bn: Vec<i64> = (0..n)
        .map(|x| {
            if n % 2 == 0 {
                even(x, n - 1)
            } else {
                odd(x, n - 1)
            }
        })
        .collect();
    let mut cn: Vec<i64> = (0..n)
        .map(|x| {
            if n % 2 == 0 {
                even_1(x, n - 1)
            } else {
                odd_1(x, n - 1)
            }
        })
        .collect();
    bn.sort();
    cn.sort();
    let up: Vec<i64> = an.iter().zip(bn.into_iter()).map(|(a, b)| a * b).collect();
    let down: Vec<i64> = an.iter().zip(cn.into_iter()).map(|(a, b)| a * b).collect();
    // println!("{:?}",up );
    // println!("{:?}",down );
    let ans = std::cmp::max(up.into_iter().sum::<i64>(), down.into_iter().sum::<i64>());
    println!("{}", ans);
}
fn odd(x: usize, upper: usize) -> i64 {
    if x == 0 || x == upper {
        1
    } else if x % 2 == 0 {
        2
    } else {
        -2
    }
}
fn odd_1(x: usize, upper: usize) -> i64 {
    if x == 0 || x == upper {
        -1
    } else if x % 2 == 0 {
        -2
    } else {
        2
    }
}
fn even(x: usize, upper: usize) -> i64 {
    if x == 0 {
        1
    } else if x == upper {
        -1
    } else if x % 2 == 0 {
        2
    } else {
        -2
    }
}
fn even_1(x: usize, upper: usize) -> i64 {
    if x == 0 {
        -1
    } else if x == upper {
        1
    } else if x % 2 == 0 {
        -2
    } else {
        2
    }
}
