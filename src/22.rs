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
    let input = read_vec::<i32>();
    let s = read::<String>();
    let c :Vec<char> = s.chars().collect();
    let ans = match (0..input[0]).chain(input[0]+1..(input[0] + input[1] + 1)).all(|x| c[x as usize].is_digit(10) && c[input[0] as usize ] == '-'){
        true => "Yes",
        false => "No",
    };
    println!("{}",ans);
}