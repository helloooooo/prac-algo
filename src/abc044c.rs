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
    let mut n : i64 = read();
    let mut ln  = read_vec::<i64>();
    // ln.sort();
    let mut ans = 0;
    while n > 1 {
        let mut (mii1,mii2) = (0,1);
        if ln[mii1 as usize] > ln[mii2 as usize] {
            let tmp = mii1;
            mii1 = mii2;
            mii2 = tmp;
        }
        for i in 2..n {
            if ln[i] < ln[mii2] {
                mii2 = i;
            }
        }

        let t = ln[mii1] + ln[mii2];
        ans += t;
        if mii1 = n-1 {
            let tmp = mii1;
            mii1 = mii2;
            mii2 = tmp;
        }
        ln[mii1] = t;
        ln[mii2] = ln[n-1];
        n -= 1;
    }
    println!("{}",ans);
}