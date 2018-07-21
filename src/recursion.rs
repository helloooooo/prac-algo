use std::time::Instant;
struct Fibmemo {
    memo: [i64; 101],
}

impl Fibmemo {
    fn new() -> Self {
        Fibmemo { memo: [0; 101] }
    }

    fn calc(&mut self, n: i64) -> i64 {
        if n < 2 {
            return n;
        }
        if self.memo[n as usize] != 0 {
            return self.memo[n as usize];
        }
        self.memo[n as usize] = self.calc(n - 2) + self.calc(n - 1);
        self.memo[n as usize]
    }
}
fn fact(x: i32) -> i32 {
    if x == 0 { 1 } else { x * fact(x - 1) }
}

fn fib(x: i64) -> i64 {
    match x {
        0 => 0,
        1 => 1,
        _ => fib(x - 1) + fib(x - 2),
    }
}
fn fib_dp(n: i64) -> i64 {
    let mut f1 = 0;
    let mut f2 = 1;
    let mut tmp = 0;
    for _ in 0..n - 1 {
        tmp = f1 + f2;
        f1 = f2;
        f2 = tmp;
    }
    tmp
}


fn main() {
    println!("{}", fact(10));
    let mut f = Fibmemo::new();
    let fib_start = Instant::now();
    let fib_ans = fib(50);
    let fib_end = fib_start.elapsed();
    let acr_start = Instant::now();
    let acr_ans = f.calc(50);
    let acr_end = acr_start.elapsed();
    let dp_start = Instant::now();
    let dp_ans = fib_dp(50);
    let dp_end = dp_start.elapsed();
    println!(
        "normal_time:{}.{}",
        fib_end.as_secs(),
        fib_end.subsec_nanos()
    );
    println!(
        "accel_time:{}.{}",
        acr_end.as_secs(),
        acr_end.subsec_nanos()
    );
    println!("dp_time:{}.{}", dp_end.as_secs(), dp_end.subsec_nanos());
    println!("{}", fib_ans);
    println!("{}", acr_ans);
    println!("{}", dp_ans);
}
