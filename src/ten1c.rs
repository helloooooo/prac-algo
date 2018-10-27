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
    let n:usize  =read();
    let mut an = vec![];
    for _ in 0..n{
        an.push(read::<i64>());
    }
    let mut ans:Vec<i64> = vec![0;n];
    an.sort();
    let mid = if n % 2 == 0  {(n/2) -1} else { n /2  };
    let mut sum = 0;
    ans[mid] = an[0];
    for j in 1..(n/2)+1{
        if j % 2 != 0 {
            ans[mid + j] = an[n-j];
            if !(n % 2 == 0 && j == n/2) { 
                ans[mid - j] = an[n-j-1];
            }
        } else {
            ans[mid+j] = an[j-1];
            if !(n % 2 == 0 && j == n/2) {
                ans[mid-j] = an[j];
            }
        }
    }
    for j in 0..(n-1){
        sum += (ans[j]-ans[j+1]).abs();
    }
    println!("{:?}",ans );
    println!("{}",sum);
    // println!( );
}
