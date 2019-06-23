use std::collections::BTreeSet;
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
    let s = read::<String>();
    let kk = read::<i32>();
    let cp = s.clone().chars().collect::<Vec<char>>();
    let mut len = cp.len();
    let mut ans:BTreeSet<String>=  BTreeSet::new();
    let mut ss = Vec::new();

    for i in 0..len{
        for j in 1..len+1{
            for k in 0..len+1{
            //    ans.insert(s.get(k..j+i).unwrap_or("").to_string());
            println!("[{}:{}]",k,j+i);
            println!("{}",s.chars().skip(k).take(j+i).collect::<String>());
            ans.insert(s.chars().skip(k).take(j+i).collect::<String>());

            }
        }
    }
    for x in ans{
        ss.push(x);
    }
    ss.sort();
    ss.reverse();
    ss.pop();
    let anslen = ss.len();
    println!("{}",ss[(anslen as i32 -kk) as usize]);
}