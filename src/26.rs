
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
    let n = read::<i32>();
    let v = prime(55555);
    let mut count  = 0;
    let size:i32 = v.len() as i32;
    for i in n..(size-n-1){
        let sub = v.clone();
        let cut = sub.split_at((n+count) as usize);
        let s:i32 = cut.0.iter().sum();
        let stri = s.to_string().chars().collect::<Vec<char>>();
        let end = stri.last().unwrap();
        if end != &'1'  || end != &'3'|| end != &'7' || end != &'9'{
             for num in cut.0 {
                print!("{} ", num);
                }
            break;
        }
        count += 1;
    }
}
fn is_prime(x: i32, ps: &Vec<i32>) -> bool {
    for p in ps {
        if p * p > x { break; }
        if x % p == 0 { return false; }
    }
    true
}

fn prime(n: i32) -> Vec<i32> {
    let mut ps = vec![2];
    let mut x = 3;
    while x <= n {
        if is_prime(x, &ps) {
            ps.push(x);
        }
        x += 2;
    }
    ps
}
