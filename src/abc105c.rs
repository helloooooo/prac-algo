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
    let mut n = read::<i64>().abs()-1;
    let mut k = down(n);
    k.reverse();

    println!("{:?}",k );
    
    for x in &k {
        // if *x == 0{
        //     print!("1");
        // }else {
        //     print!("0");
        // }
        print!("{}",x);
    }
    // while k >= 0 {
    //     let t = if k == 0 {
    //         if n % 2 == 0{
    //             0
    //         } else {
    //             1
    //         }
    //     } else {
    //         double(k)
    //     };
    //     println!("{}",n );

    //     if t <= n {
    //         v.push(1);
    //         n -= t;
    //     }else {
    //         v.push(0);
    //     }
    //     k -= 1;
    // }
    // v.reverse();
    // for x in &v{
    //     print!("{}",x);
    // }
}
fn double(x:i64) -> i64{
    let mut ans = 1;
    for i in 0..x{
        ans = 2 *ans;
    }
    ans
}
fn down(x:i64) -> Vec<i64>{
    let mut sub  = x;
    let mut ans:Vec<i64> = Vec::new();
    while sub != 0{
        ans.push(sub %2);
        sub  = sub /-2;
        
    }
    ans
}