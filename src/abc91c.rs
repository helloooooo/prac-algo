
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
    let n:i32 = read();
    let mut rn:Vec<(i32,i32)> =Vec::new();
    let mut bn:Vec<(i32,i32)> =Vec::new();
    for _ in 0..n {
        let input :Vec<i32>= read_vec();
        rn.push((input[0],input[1]));
    }
    for _ in 0..n {
        let input :Vec<i32>= read_vec();
        bn.push((input[0],input[1]));
    }
    // let mut v: Vec<i32> = Vec::new();
    let mut sub = 0;
    for i in 0..n{
        let mut count = 0;
        for j in 0..n{
            if rn[j as usize].0  < bn[i as usize].0 && rn[j as usize].1  < bn[i as usize].1{
                count +=1;
            }

        }
        if count >= i + 1{
                sub +=1;
        }
    }
    // let ans = v.iter().filter(|&&x| x ==  0).count();
    println!("{}",sub);
}