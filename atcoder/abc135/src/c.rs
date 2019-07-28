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
use std::collections::HashMap;
use std::cmp::{max,min};
fn main(){
    input!{
        n:usize,
        mut an:[i64;n+1],
        bn:[i64;n],
    }
    let mut ans =0 ;
    for i in 0..n{
        if an[i] <= bn[i]{
            ans += an[i];
            if an[i+1] >= (bn[i]-an[i]){
                ans += bn[i]-an[i];
                an[i+1] -= (bn[i]-an[i]);
            } else {
                ans += an[i+1];
                an[i+1] = 0;
            }
            an[i] = 0;
        } else {
            ans += bn[i];
        }
    }
    println!("{}",ans);
}


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
