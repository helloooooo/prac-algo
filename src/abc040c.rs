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
    memo: Vec<i64>,
    an: Vec<i64>,
}
impl Dp {
    fn new(n: i64, memo: Vec<i64>, an: Vec<i64>) -> Self {
        Dp { n, memo, an }
    }
    fn solve(&mut self, x: i64) -> i64 {
        let res = if x == self.n - 1 {
            0
        } else if x == self.n - 2 {
            self.solve(x + 1) + (self.an[x as usize] - self.an[(x + 1) as usize]).abs()
        } else {
            let t1 = (self.an[x as usize] - self.an[(x + 1) as usize]).abs();
            let t2 = (self.an[x as usize] - self.an[(x + 2) as usize]).abs();
            min(self.solve(x + 1) + t1, self.solve(x + 2) + t2)
        };
        self.memo[x as usize] = res;
        res
    }
}

fn main() {
    let n: i64 = read();
    let an = read_vec::<i64>();
    let memo = vec![0; (n + 1) as usize];
    let ans = Dp::new(n, memo, an).solve(0);
    print!("{}", ans);
}
