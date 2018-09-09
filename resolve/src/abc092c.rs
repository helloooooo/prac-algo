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
    let n:i64 = read();
    let mut an =  vec![0];
    an.append(&mut read_vec::<i64>());
    // let mut an = read_vec::<i64>();
    an.push(0);
    let sum:i64 = an.windows(2)
            .map(|t| (t[0]-t[1]).abs())
            .fold(0,|y,x| y+x);
    for j in 1..(n+1) as usize{
        let d1 = (an[j-1]-an[j]).abs();
        let d2 = (an[j]-an[j+1]).abs();
        let cost = (an[j-1]-an[j+1]).abs();
        println!("{}",sum-d1-d2+cost );
    }
}