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
const MOVES: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
use std::collections::BTreeSet;
fn main(){
    input!{
        n:usize,
        s:chars,
    }
    let mut sum_v = vec![0;n+1];
    // let mut sum_e = vec![0;n+1];
    for j in 1..n+1 {
        if s[j-1] == 'W'{
            sum_v[j] = sum_v[j-1] + 1;  
        } else {
            sum_v[j] = sum_v[j-1];
        }
    }
    let mut ans = usize::max_value();
    for j in 0..n{
        let count = sum_v[j]+n-(j+1) -(sum_v[n]-sum_v[j+1]);
        ans = min(count,ans);
    };
    println!("{}",ans);
}