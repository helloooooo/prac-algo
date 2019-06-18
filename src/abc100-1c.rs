 
fn main() {
    input! {
        n:usize,
        an:[usize;n],
    }
    let ans: usize = an
        .iter()
        .filter(|&x| x % 2 == 0)
        .map(|&x| calc(x))
        .fold(0, |y, x| y + x);
    println!("{:?}", ans);
}
fn calc(x: usize) -> usize {
    let mut s = x.clone();
    let mut cnt = 0;
    while s % 2 == 0 {
        s /= 2;
        cnt += 1;
    }
    cnt
}
