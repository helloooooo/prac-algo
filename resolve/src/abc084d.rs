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
use std::collections::HashMap;
fn main(){
    input!{
        q:i64,
        lrn:[(i64,i64);q],
    }
    println!("{:?}",sieve(1000));
}
fn sieve(n: usize) -> Vec<usize> {
    let mut ps: Vec<usize> = vec![2];
    let mut xs: Vec<bool> = vec![true; n / 2];
    let mut x = 3;
    while x * x <= n {
        let mut y = (x - 3) / 2;
        if xs[y] {
            ps.push(x);
            y += x;
            while y < xs.len() {
                xs[y] = false;
                y += x;
            }
        }
        x += 2;
    }
    while x <= n {
        if xs[(x - 3) / 2] { ps.push(x); }
        x += 2;
    }
    ps
}
