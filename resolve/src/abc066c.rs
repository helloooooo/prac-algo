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
    let n:i64 = read();
    let an = read_vec::<i64>();
    let (mut odd,mut even) = an.into_iter().enumerate()
        .fold((vec![],vec![]),|(mut bn,mut cn),(i,x)|{
            if (i+1) % 2 == 0 {
                cn.push(x);
            } else {
                bn.push(x);
            }
            (bn,cn)
        });
    let ans = if n % 2 == 0 {
        even.reverse();
        even.append(&mut odd);
        even
    } else {
        odd.reverse();
        odd.append(&mut even);
        odd
    };
    let ans:Vec<String> = ans.into_iter()
        .map(|x|x.to_string()).collect();
    println!("{}",ans.connect(" "));
}