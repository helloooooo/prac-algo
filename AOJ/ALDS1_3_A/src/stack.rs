macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
    ($next:expr, mut $var:ident : $t:tt $($r:tt)*) => {
        let mut $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}
fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}
fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}
use std::collections::HashMap;
use std::cmp::{max,min};
fn main(){
    let s = read_vec::<String>();
    // println!("{:?}",s);
    let ans = solve(& s);
    print!("{}",ans);
}


fn solve(s:&Vec<String>) -> i64 {
    let mut stack = vec![];
    let mut operator = vec!["+".to_string(),"*".to_string(),"-".to_string()];
    for c in s {
        if operator.contains(&c) {
            let right = stack.pop().unwrap();
            let left = stack.pop().unwrap();
            let res = match (&c as &str) {
                "*" => left * right,
                "+" => left + right,
                "-" => left - right,
                _ => unimplemented!(),
            };
            stack.push(res);
        } else {
            let value = c.parse::<i64>().unwrap();
            stack.push(value);
        }
    }
    let res = stack.pop().unwrap();
    res
}

// fn calc(left:i64,right:i64,ope:&'static str) -> i64 {
//     let res = match ope {
//         "*" => left * right,
//         "+" => left + right,
//         "-" => left - right,
//         _ => unimplemented!(),
//     };
//     res
// }

fn is_prime(x:i64) -> bool {
    if x == 2 {return true;}
    if x < 2 || x % 2 == 0 {return false;}
    let mut j = 3;
    while j <= (x as f64).sqrt() as i64 {
        if x % j == 0 {
            return false;
        }
        j += 2;
    }
    true
}



#[test]
fn stack_test_no1(){
    let s = vec!['1','2','+','3','4','-','*'];
    println!("{}",solve(&s));
    assert_eq!(solve(&s),-3)
}
#[test]
fn stack_test_no2(){
    let s = vec!['1','2','+'];
    assert_eq!(solve(&s),3)
}
#[test]
fn stack_test_no3(){
    let s = vec!['1',' ','1',' ','+'];
    assert_eq!(solve(&s),2)
}
