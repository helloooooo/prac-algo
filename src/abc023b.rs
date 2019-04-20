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
        n:usize,
        s:String,
    }
    if n % 2 == 0{
        println!("-1");
        return;
    }
    let mut cnt = 1;
    let mid  = (n-1)/ 2;
    let mut v = VecDeque::new();
    v.push_back('b');
    for j in 0..mid{
        if cnt % 3 == 1 {
            v.push_front('a');
            v.push_back('c');
        } else if cnt % 3 == 2{
            v.push_front('c');
            v.push_back('a');
        } else {
            v.push_front('b');
            v.push_back('b');
        }
        cnt += 1;
    }
    let sub:String = v.iter().map(|&c|c).collect();
    println!("{}", if s == sub {mid as i64} else {-1});
}