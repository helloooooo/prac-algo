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
    let s : Vec<char> = read::<String>().;

}
fn solve(sum:i64,s1:String,s2:String) -> i64 {
    if s2.len() ==  0 {
        let s1 = s1.chars().take(1).unwrap();
        let s2 = s1.chars().skip(1).unwrap();
        solve(sum,s1,s2);
    } else if s1.len() == 0 {
        sum + s2.parse::<i64>().unwrap()
    } else {
        let sum1 = {
            let sum  = sum + s1.parse::<i64>();
        }
    }

}

fn f(s1: String, s2: String, sum: i64) -> i64 {
    if s2.len() == 0 {
        let s2 = s1.clone().chars().take(1).collect::<String>();
        let s1 = s1.clone().chars().skip(1).collect::<String>();
        f(s1, s2, sum)
    } else if s1.len() == 0 {
        sum + s2.parse::<i64>().unwrap()
    } else {
        //s2を使う
        let sum1 = {
            let sum = sum + s2.parse::<i64>().unwrap();
            let s2 = String::new();
            f(s1.clone(), s2, sum)
        };
        //s2を使わない
        let sum2 = {
            let s2 =
                format!("{}{}", s2, s1.clone().chars().take(1).collect::<String>()).to_string();
            let s1 = s1.clone().chars().skip(1).collect::<String>();
            f(s1, s2, sum)
        };
        sum1 + sum2
    }
}