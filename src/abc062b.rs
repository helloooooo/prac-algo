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
    let hw = read_vec::<u64>();
    let (h,w)  = (hw[0],hw[1]);
    let mut v = Vec::new();
    for _ in 0..h{
        v.push(read::<String>());
    }
    // let v = read_vec2::<String>(h as u32);
    printShrap(&w);
    for s in &v{
        println!("#{}#",s);
    }
    printShrap(&w);
}
fn printShrap(w:&u64){
    for _ in 0..2+*w{
        print!("#");
    }
    println!("")
}