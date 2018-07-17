
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
fn  main() {

    let n = read::<i32>();
    let mut an:Vec<i32> = Vec::new();
    an.push(0);
    // for _ in 0..n {
    //     an.push(read::<i32>());
    // }
    let mut sub= read_vec::<i32>();
     an.append(&mut sub);
    an.push(0);
    let mut costmas = 0;
    // let mut ans = Vec::new();
    for i in 1..n+2{
            let cost  = (an[i as usize-1] - an[i as usize]).abs();
            
            costmas += cost;
    }
   
    for i in 1..n+1 {
        let sub = (an[i  as usize-1] - an[i  as usize] ).abs();
        let div1 = (an[i as usize] - an[i as usize + 1]).abs();
        let div2 = (an[i as usize-1] - an[i as usize + 1]).abs();
        println!("{}",costmas-sub - div1+ div2);
    }

}