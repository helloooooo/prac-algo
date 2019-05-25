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
use std::cmp::{max,min};
fn main(){
    input!{
        n:i64,
        m:i64,
        mut an:[i64;n],
        mut bcm:[(i64,i64);m],
    }
    an.sort();
    bcm.sort_by_key(|x|x.1);
    bcm.reverse();
    let mut sum:i64 = an.iter().sum();
    // // let change_sum = bcm.iter().map(|x|x.0).sum();
    // println!("{:?}",bcm);
    let mut change_index = 0;
    // let mut min = an.iter().min().unwrap();
    // println!("{}",sum);
    'outer: for &(mut b,c) in &bcm {
        if an[change_index] >= c {
            break;
        }
        while b > 0 {
            if an[change_index] >= c {
                break 'outer;
            }
            sum += c-an[change_index];
            // println!("{}",sum);
            b -= 1;
            change_index +=1;
            if change_index >= n as usize {
                break 'outer;
            }
        }
    }
    println!("{}",sum);
}