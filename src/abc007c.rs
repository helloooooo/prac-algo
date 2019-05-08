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
    input! {
        r:usize,
        c:usize,
        mut s:(usize,usize),
        mut g:(usize,usize),
        field:[chars;r],
    }
    s.0 -= 1;
    s.1 -= 1;
    g.0 -= 1;
    g.1 -= 1;
    let mut que = BinaryHeap::new();
    let mut dist = vec![vec![std::i32::MAX; c]; r];
    dist[s.0][s.1] = 0;
    que.push(s);
    while let Some(point) = que.pop() {
        let d = dist[point.0][point.1];
        for &mv in &MOVES {
            let after = (point.0 as i32 + mv.0, point.1 as i32 + mv.1);
            let after = (after.0 as usize, after.1 as usize);
            if after.0 < r && field[after.0][after.1] == '.' && dist[after.0][after.1] > d + 1 {
                dist[after.0][after.1] = d + 1;
                que.push(after);
            }
        }
    }
    println!("{}", dist[g.0][g.1]);
}
