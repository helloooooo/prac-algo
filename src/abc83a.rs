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

fn main() {
    let i : Vec<i32> = read_vec();
    let l = i[0] + i[1];
    let r = i[2] + i[3];
    let diff = l -r ;
    if diff == 0 {
        println!("Balanced");
    } else if diff < 0 {
        println!("Right");
    } else  {
        println!("Left");
    }

}
