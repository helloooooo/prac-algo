
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
fn main(){
    let inp = read_vec::<i32>();
    let n = inp[0];
    let m = inp[1];
    let x = inp[2];
    let an = read_vec::<i32>();
    let xn  = vec![0..m];
    let ans_n = an.iter().filter(|&a| a > &x && a < &n).count();
    let ans_0 = an.iter().filter(|&a| a < &x && a > &0).count();
    println!("{}",if ans_n > ans_0 { ans_0} else {ans_n});

    
}