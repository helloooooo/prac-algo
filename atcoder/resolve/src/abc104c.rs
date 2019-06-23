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
    let ans:String = if n == 0 {
        "0".to_string()
    } else {
        let mut n = n;
        let mut v = vec![];
        while n != 0 {
            if n % 2 == 0{
                v.push('0');
                n = n / -2;
            } else {
                v.push('1');
                n = (n-1) / -2;// n / 2 = 1 =>  (n -1 )/ 2= 0
            }
        }
        v.reverse();
        v.into_iter().collect::<String>()
    };
    println!("{}", ans);
}