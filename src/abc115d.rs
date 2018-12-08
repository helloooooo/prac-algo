use std::cmp::min;
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
    let (n,x) = {
        let t = read_vec::<i64>();
        (t[0],t[1])
    };
    let mut p = 3;
    let mut b = 2;
    for _ in 0..n-1{
        p = p*2 + 1;
        b = b*2 + 2;
    }
    // let test = create_string(n);
    let mut rec = Recv::new();
    let ans =rec.create_string(n);
    println!("{}",test);
    let max = p+b;
    let ans = if max-1/2 > x {
        (x/5)*3 + "BPPPB".chars().take((x % 5) as usize).filter(|&a| a == 'P').count() as i64
    } else {
        let sub = x-1;
        1 + (sub/5) *3 + "BPPPB".chars().take((sub % 5) as usize).filter(|&a| a == 'P').count() as i64
    };
    println!("{}",ans);
}
fn create_string(n:i64,memo:Vec<String>) -> String{
    if n == 1 {
        "BPPPB".to_string()
    } else {
        let l = create_string(n-1);
        format!("B{}P{}B",l,l)
    }
}
struct Recv {
    memo: [i64;50],
}
impl Recv{
    fn new() -> Self {
        Recv {
            memo: [1; 50]
        }
    }
    fn create_string(&mut self,n:i64) -> String{
        if n == 1 { return "BPPPB".to_string(); }
        if self.memo[n as usize] != 1 { return self.memo[n as usize];}
        let t = self.create_string(n-1);
        self.memo[n as usize] = format!("B{}P{}B",t,t).parse::<i64>();
        self.memo[n as usize]
    }
}