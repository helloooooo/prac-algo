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
    let nm:Vec<u32> = read_vec();
    let am:Vec<Vec<u32>> = read_vec2(nm[1]);
    let mut oi = vec![false;(nm[0]+ 1) as usize];
    let mut iton = vec![false;(nm[0] + 1) as usize];
    for i in 0..nm[1]{
        if am[i as usize][0] == 1 {
            oi[(am[i as usize][1]) as usize] = true;
        } else if am[i as usize][1] == nm[0] {
            iton[(am[i as usize][0]) as usize] = true; 
        }
    }
    for i in 0..nm[0] + 1 {
        if oi[i as usize] && iton[i as usize]{
            println!("POSSIBLE");
            return;
        }
    }
    println!("IMPOSSIBLE");
}