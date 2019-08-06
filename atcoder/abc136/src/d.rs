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
        s:chars,
    }
    let mut rlv = vec![];
    let mut sum = 0;
    for i in 0..s.len()-1 {
        if s[i] == 'R' &&  s[i+1] == 'L' {
            rlv.push(i);
        }
    }
    let mut ans = vec![0;s.len()];
    for i in 0..rlv.len() {
        let mut left = rlv[i];
        let mut right = left + 1;
        while left > 0 && s[left-1] == 'R' {
            left -= 1;
        }
        while right + 1 < s.len() && s[right + 1] == 'L' {
            right += 1;
        }
        let mut lv = 0;
        let mut rv = 0;
        for j in left..rlv[i] {
            if (rlv[i]-j) % 2 == 0 {
                lv += 1;
            } else {
                rv += 1;
            }
        }
        for j in rlv[i]..(right+1) {
            if (j-rlv[i]) % 2 == 0 {
                lv += 1;
            } else {
                rv += 1;
            }
        }
        ans[rlv[i]] = lv;
        ans[rlv[i] + 1] = rv;
    }
    for i in 0..s.len() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", ans[i]);
    }
    println!();
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
