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
const DIV:usize = 1000000007;
fn main(){
    input!{
        h:usize,
        w:usize,
        sn:[chars;h],
    }
    let mut field:Vec<Vec<usize>> = vec![vec![0;w];h];
    for j in 0..h {
        let mut done = vec![false;w];
        for k in 0..w {
            if sn[j][k] == '#'{continue;}
            if done[k] {continue;}
            let mut c = 0;
            while k+c<w {
                if sn[j][k+c] == '#' {break};
                c +=1;
            }
            for l in 0..c {
                field[j][k+l] += c;
                done[k+l] = true;
            }
        }
    }
    for j in 0..w {
        let mut done = vec![false;h];
        for k in 0..h {
            if sn[k][j] == '#'{continue;}
            if done[k] {continue;}
            let mut c = 0;
            while k+c< h {
                if sn[k+c][j] == '#' {break};
                c +=1;
            }
            for l in 0..c {
                field[k+l][j] += c;
                done[k+l] = true;
            }
        }
    }
    let mut ans = 0;
    for j in 0..h {
        for k in 0..w {
            ans = max(ans,field[j][k]);
        }
    }
    println!("{}",ans-1);
}