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
use std::cmp::{min,max};
use std::collections::HashMap;
fn main() {
    input! {
        n:usize,
        an:[i64;n],
    }
    if an.iter().all(|&x| x==an[0]){
        println!("{}",n/2);
        return;
    }
    let mut odd_map = HashMap::new();
    let mut even_map = HashMap::new();
    for j in 0..n{
        if j % 2 == 0 {
            *odd_map.entry(&an[j]).or_insert(0) += 1;
        } else {
            *even_map.entry(&an[j]).or_insert(0) += 1;
        }
    }
    let mut odd:Vec<_> = odd_map.into_iter().collect();
    let mut even:Vec<_> = even_map.into_iter().collect();
    odd.sort_by_key(|x|x.1);
    even.sort_by_key(|x|x.1);
    let odd_max = odd.last().unwrap();
    let even_max = even.last().unwrap();
    let odd_2 = if odd.len() == 1 {
        0
    } else{
        odd[odd.len()-2].1
    };
    let even_2 = if even.len() == 1{
        0
    } else {
        even[even.len()-2].1
    };
    let ans = if odd_max.0 == even_max.0 {
        let a1 = n- odd_max.1- even_2;
        let a2 = n- even_max.1 -odd_2;
        min(a1,a2)
    } else{
        n-odd_max.1-even_max.1
    };
    println!("{}",ans);
}