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
    let v:Vec<char> = s.chars().collect();
    let master:Vec<&str>= vec!["Do","W","Re","W","Mi","Fa","W","So","W","La","W","Si"].into_iter().rev().collect();
    let x = "WBWBWWBWBWBW".to_string();
    for j in 0..14{
        let t= v.clone().into_iter().skip(j).take(12).collect::<String>();

        if t == x {
            match j {
                0 => println!("Do"),
                _ => println!("{}",master[(j-1) as usize]),
            }
            return;
        }
    }
}