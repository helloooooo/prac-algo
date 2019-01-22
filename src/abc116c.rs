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
    let n = read::<i64>();
    let hn = read_vec::<i64>();
    let mut count:Vec<i64> = vec![0;n as usize];
    let mut ans = 0;
    for j in 0..n {
        let sub = hn.iter().enumerate().filter(|&x| x.1 < &hn[j as usize]).nth(0);
         match sub {
            Some(i) => {
                ans += (hn[j as usize] - count[j as usize]).abs();
                for k in j..*i.1 {
                    count[k as usize] = (hn[ j as usize] -count[k as usize] ).abs();
                }
            },
            None => {
                ans += (hn[j as usize] - count[j as usize]).abs(); 
                count  = count.iter().map(|&x| (x-hn[j as usize]).abs()).collect::<Vec<i64>>();
            },
        }
            println!("{:?}",count);
    }
    let max = hn.iter().max().unwrap();
    let ans = hn.iter().fold(0,|y,x| y+(x-max).abs());

}