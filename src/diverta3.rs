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
        n:i64,
        sn:[String;n],
    }
    let mut end_a = 0;
    let mut start_b = 0;
    let mut all = 0;
    let mut ans = 0;
    for j in 0..sn.len() {
        if sn[j].starts_with("B") && sn[j].ends_with("A"){
            all += 1;
        } else if sn[j].ends_with("A") {
            end_a += 1;
        } else if sn[j].starts_with("B"){
            start_b += 1;
        }
        let cn:Vec<char> = sn[j].chars().collect();
        for k in 0..cn.len()-1 {
            if cn[k] == 'A' && cn[k+1] =='B' {
                ans += 1;
            }
        }
    }
    if all == 0 {
        ans += min(end_a, start_b);
    } else if all != 0 && end_a + start_b > 0 {
        ans += all + min(end_a, start_b );
    } else if all != 0 {
        ans += all -1;
    }
    println!("{}",ans );
}
