use std::cmp::min;
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
    let n = read::<u32>();
    let mut v = Vec::new();
    for _ in 0..n {
        v.push(read::<String>());
    }
    let mut  ans_vec = vec![51;26];
    for i in 0..n{
        let a_size = 'a' as usize;
        let sub = v[i as usize].chars().fold(vec![0;26],|mut x,y| {x[y as usize - a_size] +=1; x});
        for i in 0..26 { ans_vec[i] = min(sub[i],ans_vec[i]); }
    }
    let ans:String = ans_vec.iter().enumerate().flat_map(|(i,x)| vec![(i as u8 + 'a' as u8) as char;*x]).collect();
    println!("{}",ans);
}