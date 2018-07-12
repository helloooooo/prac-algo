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
pub const M: i32 = 1000000007;
fn main() {
    let ans = solve();
    println!("{}",ans );
}
fn solve() -> i32{
    let n = read::<i32>();
    let mut an = read_vec::<i32>();
    let len:i32 = an.len() as i32;

    let mut v:Vec<i32>  =Vec::new();
    let mut ans:i32= 1;
    for i in 0..len/2{
        v.push(len -(2*i + 1));
        v.push(len -(2*i + 1));
    }
    if len  % 2 != 0{
        v.insert(0, 0);
    }
    an.sort();
    v.sort();
    let flag = an.iter().zip(v.iter()).fold(false,|x,y| if y.0 == y.1 {
        x
    }else {
        true
    });
    if flag == true{
        0
    } else {
        for _ in 0..len/2{
            ans = ans *2 % M;
        }
        ans
    }
}
