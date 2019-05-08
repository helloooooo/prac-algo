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
fn main() {
    input! {
        n:usize,
        m:usize,
        lrn:[(usize,usize,i64);n],
    }
    let mut count = vec![0; m + 1];
    let mut cost: Vec<i64> = vec![0; m + 1];
    for &(l, r, c) in &lrn {
        count[l - 1] += 1;
        count[r] -= 1;
        cost[l - 1] += c;
        cost[r] -= c;
    }
    let mut sum_count = vec![0];
    let mut sum_cost = vec![0];
    count.insert(0, 0);
    cost.insert(0, 0);
    for j in 0..m + 1 {
        let s = sum_count[j] + count[j + 1];
        let c = sum_cost[j] + cost[j + 1];
        sum_count.push(s);
        sum_cost.push(c);
    }
    println!("{:?}", sum_count);
    println!("{:?}", sum_cost);
}
