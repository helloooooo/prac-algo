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
    let (n,k) = {
        let t = read_vec::<i64>();
        (t[0],t[1])
    };
    let cn = vec!['0','1','2','3','4','5','6','7','8','9'];
    let dn:Vec<char> = read::<String>().chars().filter(|&x| x != ' ').collect();
    let use_n:Vec<char>  = cn.into_iter().filter(|x| !dn.contains(&x)).collect();

    // println!("{:?}",use_n );
    for j in n..{
        let num = j;
        let s = num.to_string();
        if s.chars().all(|x| use_n.contains(&x)){
            println!("{}",s );
            return;
        }
    }
}