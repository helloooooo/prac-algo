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
    let n:usize = read();
    let mut an = vec![vec![0,0,0]];
    for _ in 0..n{
        an.push(read_vec::<i64>());
    }
    let ans:bool = (0..n).all(|i|{
        let move_diff = an[i+1][0] -an[i][0];
        let point_diff = (an[i+1][1]-an[i][1]).abs() + (an[i+1][2]-an[i][2]).abs();
        (point_diff <= move_diff && point_diff % 2 == move_diff % 2)
    });
    println!("{}" ,if ans { "Yes"} else {"No"});
}