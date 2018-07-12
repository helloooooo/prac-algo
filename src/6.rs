
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
    let n:u32 = read();
    let mut a = read_vec::<u32>();
    a.sort_by(|x,y| y.cmp(x));
    let mut alice = 0;
    let mut bob = 0;
    for (i, &x) in a.iter().enumerate() {
         if i % 2 == 0 {
             alice += x;
         } else {
             bob += x;
         }
    }

    println!("{}", alice - bob);
}