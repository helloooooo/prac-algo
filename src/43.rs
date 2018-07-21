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
    let n  = read::<i32>();
    let s = read::<String>();
    let cs = s.chars().collect::<Vec<char>>();
    let mut ans = Vec::new();
    for i in 1..n{
        let mut sub = cs.clone();
        let mut cut = sub.split_at(i as usize);
        let mut a1 = cut.0.to_vec();
        let mut a2= cut.1.to_vec();
        a1.sort();
        a2.sort();
        a1.dedup();
         a2.dedup();
        // let a1:Vec<char> = cut.0.to_vec().dedup().collect();
        // let a2:Vec<char> = cut.1.to_vec().dedup().collect();
        let count = a1.iter().fold(0,|x,y| {match  a2.contains(y) {
                true => x + 1,
                false => x,
        }});

        ans.push(count);    
    }
    // ans.sort_by_key(|&(a,b)|b);
    println!("{}",ans.iter().max().unwrap());
}
