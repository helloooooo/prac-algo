use std::cmp::{max, min};
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
const MOVES: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
use std::collections::BinaryHeap;
fn main() {
    input!{
        w:i64,
        h:i64,
    }
    // let max = max(w,h);
    // let min = min(w,h);
    let up:i64 = (1..w+h-2).fold(1,|y,x| {calc(y,x, 1e5 as i64 + 7) });
    println!("{}",up);
    let left:i64 = (1..h-1).fold(1,|y,x| {calc(y,x, 1e5 as i64 + 7)});
    let right:i64 = (1..w-1).fold(1,|y,x| {calc(y,x, 1e5 as i64 + 7)});
    println!("{}", up/(left*right));
}
fn calc(a:i64,b:i64,p:i64) -> i64 {
    if b == 0 {
        return 1;
    }
    if b % 2 == 0 {
        let d = calc(a,b/2,p);
        (d * d) % p
    } else {
        (a*calc(a,b-1,p)) % p
    }
}