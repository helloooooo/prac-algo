use std::cmp::{min,max};
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
use std::collections::VecDeque;
fn main(){
    input!{
        n:i64,
        s:chars,
    }
    let mut s = s;
    s.dedup();
    let mut black:i64 = s.iter().filter(|&c| *c=='#').count() as i64;
    let mut white = s.len() as i64 - black;
    let n = s.len() as i64;
    let mut flag = false;
    let mut sub = false;
    let mut count = 0;
    // let mut white = 0;
    
    for j in 0..n {

        if flag {
            if s[j as usize] == '.' {
                white += 1;
            }
            if s[(j-1) as usize] == '#' && s[j as usize] == '#'{
                count += 1;
                sub = true;
            } else {
                count = 0;
                sub = false;
            }
        }
        if s[j as usize] == '#' && !flag{
            flag = true;
        }
        
    }
    if sub{
        black -= count;
    }
    if s[(n-1) as usize] == '#' {
        black -= 1;    
    }


    let ans = min(white,black);
    println!("{}",ans);
}
