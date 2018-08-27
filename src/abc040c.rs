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

struct Dp {
    n: i64,
    memo:Vec<i64>,
    an:Vec<i64>,
}
impl Dp {
    fn new(n:i64,memo:Vec<i64>,an:Vec<i64>) -> Self {
        Dp { n,
            memo,
            an,
        }
    }
    fn solve(&mut self ,x : i64) -> i64{
        if x == 1 {
            self.memo[x as usize] = (self.an[x as usize] - self.an[(x-1) as usize]);
        }
        if x == self.n {
            return self.memo[self.n as usize]
        } else if x == 1 {
            
            // self.memo[x as usize]
        } else {
            let t1 = self.memo[(x-1) as usize] + (self.an[(x-1) as usize] - self.an[(x) as usize]).abs();
            let t2 = self.memo[(x-2) as usize] + (self.an[(x-2) as usize] - self.an[(x) as usize]).abs();
            self.memo[x as usize] =  min(t1,t2);
            self.solve(x+ 1)
        }
    }
}

fn main(){
    let n : i64 = read();
    let an = read_vec::<i64>();
    let memo = vec![0;(n+1) as usize];
    let ans = Dp::new(n,memo,an).solve(1);
    print!("{}",ans);
}
