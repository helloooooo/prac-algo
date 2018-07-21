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
    let input: Vec<i32> = read_vec();

    let n:i32 = input[0];
    let x:i32 = input[1];
    let mut ans = None;
    'outer: for i in 0..n+1 {
        for j in 0..n-i + 1{
            let k = n -i - j;
            if i * 10000 + j * 5000 + k *1000 == x {
                ans = Some((i,j,k));
                break 'outer;
            }
        }
    }
    let (x,y,z) = ans.unwrap_or((-1,-1,-1));
    println!("{} {} {}",x,y,z);
}