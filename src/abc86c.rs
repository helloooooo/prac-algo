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
    let n : u32 = read();
    let tn :Vec<Vec<i32>> = read_vec2(n);
    let s:Vec<(i32,i32,i32)> = tn.iter().map(|x| (x[0],x[1],x[2]) ).collect();
    let mut n_point = (0,0,0);
    for (time,x,y) in s {
        for _ in 0..(time - n_point.0)  {
            if x < n_point.1 {
                n_point.1 -= 1;
            } else if y < n_point.2 {
                n_point.2 -= 1;
            } else if x > n_point.1 {
                n_point.1 += 1;
            } else if y > n_point.2 {
                n_point.2 += 1;
            } else {
                n_point.1 -= 1;
            }
            n_point.0 += 1;
        }
        if n_point != (time,x,y) {
            println!("No");
            return
        }
    }
    println!("Yes");
}