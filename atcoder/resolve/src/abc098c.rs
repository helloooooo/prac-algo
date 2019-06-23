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
    let s:Vec<char> = read::<String>().chars().collect();
    let sum:Vec<_> = s.clone().iter().map(|x| if *x == 'E'{ 1 } else { 0 })
        .scan(0,|state,x|{
            *state = *state + x;
            Some(*state)
        }).collect();
    let ans = (0..n).fold(1e9 as i64, |res,j| {
        let diff = if j == 0{
            (sum[(n-1) as usize] - sum[j as usize])
        } else if j == (n-1){
            j - sum[(j-1) as usize]
        } else {
            ((sum[(n-1) as usize] - sum[(j) as usize])) + (j+1 - sum[j as usize])
        };
        std::cmp::min(res, diff)
    });
    println!("{}",ans );
}