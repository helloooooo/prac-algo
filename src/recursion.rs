use std::time::Instant;
struct Fibmemo{
    memo: [i64;101],
}

impl Fibmemo{
        fn new() -> Self {
        Fibmemo { memo: [0; 101] }
    }

    fn calc(&mut self, n: i64) -> i64 {
        if n < 2 { return n; }
        if self.memo[n as usize] != 0 { return self.memo[n as usize]; }
        self.memo[n as usize] = self.calc(n-2) + self.calc(n-1);
        self.memo[n as usize]
    }
}
fn fact(x: i32) -> i32 {
    if x == 0 { 1 } else { x * fact(x - 1) }
}

fn fib(x: i64) -> i64 {
    if x <= 1 { x } else { fib(x - 1) + fib(x - 2) }
}



fn main() {
    println!("{}", fact(10));
    let mut f = Fibmemo::new();
    let fib_start = Instant::now();
    let fib_ans = fib(40);
    let fib_end = fib_start.elapsed();
    let acr_start = Instant::now();
    let acr_ans = f.calc(40);
    let acr_end = acr_start.elapsed();
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
    println!("{}",acr_ans);
}
