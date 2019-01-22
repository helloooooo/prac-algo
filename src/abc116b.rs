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
    let s = read::<i64>();
    let mut ans:Vec<i64> = vec![];
    let mut count = 1;
    let mut m = 1 ;
    loop {
        
        if count == 1 {

            count = s;
        } else if count % 2 == 0 {

            count /= 2;
        } else {

            count = 3*count + 1;
        }
        if ans.contains(&count) {
            break;
        }
        ans.push(count);
        m += 1;
    }
    println!("{:?}",ans);
    println!("{}",m);
}
