use std::cmp::{max, min};

fn solve(n: i32, len: i32, x: Vec<i32>) -> (i32, i32) {
    let mut min_t = 0;
    //x.iter().map(|v| minT = max(minT, min(*v, len - v)));
    for i in 0..x.len() {
        min_t = max(min_t, min(x[i], len - x[i]))
    }
    let mut max_t = 0;
    //x.iter().map(|v| maxT = max(maxT, max(*v, len - v)));
    for i in 0..x.len() {
        max_t = max(max_t, max(x[i], len - x[i]))
    }
    (max_t, min_t)
}
fn main() {
    let (mat, mit) = solve(3, 10, vec![2, 6, 7]);
    println!("max:{},min:{}", mat, mit);
}
