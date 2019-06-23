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
use std::cmp::max;
use std::collections::HashMap;
fn main(){
    input!{
        n:usize,
        sn:[chars;n],
    }
    let mut map = HashMap::new();
    for s in sn {
        *map.entry(s[0]).or_insert(0) += 1;
    }
    let master:Vec<i64> = "MARCH".chars().map(|c| *map.entry(c).or_insert(0)).collect();
    // println!("{:?}",master);
    let mut ans = 0;
    for bit in 0usize..(1<<5) {
        if bit.count_ones() != 3 {
            continue;
        }
        let mut sub = 1; 
        for j in 0..5 {
            if  bit&(1<<j) != 0 {
                sub *= master[j];
            }
        }
        ans += sub;
    }
    println!("{}",ans);
}