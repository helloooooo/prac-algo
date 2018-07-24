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
    let xy = read_vec::<u64>();
    let (x,y) = (xy[0]-1,xy[1]-1);
    let v = vec![1,3,1,2,1,2,1,1,2,1,2,1];
    let ans = if (v[x as usize] == v[y as usize]){
        "Yes"
    } else{
        "No"
    };
    println!("{}",ans );
}