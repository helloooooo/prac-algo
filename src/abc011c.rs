
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
// greedy
fn main(){
    input!{
        mut n:i64,
        ng:[i64;3],
    }
    let mut flag = true;
    for _ in 0..100 {
        if n == 0 {
            break;
        } else if !check(&ng, n){
            flag = false;
            break;
        }
        if n-3 >= 0 && check(&ng, n-3) {
            n -= 3;
        } else if n-2 >=0  && check(&ng, n-2){
            n -= 2;
        } else if n -1 >= 0 && check(&ng, n-1){
            n-=1;
        } else {
            flag = false;
            break;
        }
    }
    if n != 0 {
        flag = false;
    }
    println!("{}",if flag {"YES"} else {"NO"});
}
fn check(ng:&[i64],target:i64)-> bool{
    !ng.contains(&target)
}