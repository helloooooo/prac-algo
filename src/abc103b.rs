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
    let s : String= read();
    let t : String = read();
    if s == t {
            println!("Yes");
            return;
    }
    let mut cs:Vec<char>  =s.chars().collect();
    let len = cs.len();
    for i in 0..cs.len(){
        let last = cs[(len-1)as usize];
        let mut  ans:Vec<char> = cs.clone().into_iter().take(len-1).collect();
        ans.insert(0, last);
        
        if (t.chars().collect::<Vec<char>>() ==  ans){
            println!("Yes");
            return;
        }
        cs = ans;
    }
    println!("No");
}