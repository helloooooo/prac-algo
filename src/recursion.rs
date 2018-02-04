use std::time::Instant;

fn fact(x: i32) -> i32 {
    if x == 0 { 1 } else { x * fact(x - 1) }
}

fn fib(x: i32) -> i32 {
    if x <= 1 { x } else { fib(x - 1) + fib(x - 2) }
}

fn acceralate_fib(x: usize, memo: &mut Vec<Option<i64>>) -> i64 {
    // メモ探索を用いた高速化のつもりがそんな早くない
    if x <= 1 {
        return 1;
    }
    if let Some(y) = memo[x] {
        return y;
    }
    let retval = acceralate_fib(x - 1, memo) + acceralate_fib(x - 2, memo);
    memo[x] = Some(retval);
    retval
}


fn main() {
    println!("{}", fact(10));
    let fib_start = Instant::now();
    let fib_ans = fib(10);
    let fib_end = fib_start.elapsed();
    let mut v: Vec<Option<i64>> = vec![Some(0); 11];
    let acr_start = Instant::now();
    let acr_ans = acceralate_fib(10, &mut v);
    let acr_end = acr_start.elapsed();
    println!(
        "normal_time:{}.{}",
        fib_end.as_secs(),
        fib_end.subsec_nanos()
    );
    println!(
        "accel_time:{}.{:03}",
        acr_end.as_secs(),
        acr_end.subsec_nanos()
    );
}
