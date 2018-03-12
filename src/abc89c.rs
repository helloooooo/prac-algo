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
    let n :u32 = read();
    let mut sn: Vec<String> = Vec::new(); // M A R C H 探索
    for _ in 0..n {
        sn.push(read::<String>())
    }
    let mut counts:Vec<f64> = vec![0.,0.,0.,0.,0.];
    let first :Vec<usize> = vec![0,0,0,0,0,0,1,1,1,2];
    let second:Vec<usize> = vec![1,1,1,2,2,3,2,2,3,3];
    let third :Vec<usize> = vec![2,3,4,3,4,4,3,4,4,4];
    for i in 0..sn.len() {
        match sn[i as usize].chars().nth(0).unwrap() {
            'M' => counts[0] += 1.,
            'A' => counts[1] += 1.,
            'R' => counts[2] += 1.,
            'C' => counts[3] += 1.,
            'H' => counts[4] += 1.,
            _ => (),
        }
    }
    let mut ans = 0.;
    for i in 0..10 {    
        ans += counts[first[i as usize]] * counts[second[i as usize]] * counts[third[i as usize]];
    }
    println!("{}",ans);
}