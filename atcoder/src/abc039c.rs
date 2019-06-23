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
    let s = read::<String>();
    let v: Vec<char> = s.chars().collect();
    let master: Vec<&str> = vec!["Do", "Re", "Mi", "Fa", "So", "La", "Si"];
    let index = vec![0, 2, 4, 5, 7, 9, 11];
    let x = "WBWBWW".to_string();
    let o = match s.find("WWBWBWW").unwrap() {
        11 => "Do",
        9 => "Re",
        7 => "Mi",
        6 => "Fa",
        4 => "So",
        2 => "La",
        0 => "Si",
        _ => "",
    };
    println!("{}", o);
}
