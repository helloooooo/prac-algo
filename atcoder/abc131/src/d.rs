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
use std::collections::HashMap;
use std::cmp::{max,min};
fn main(){
    input!{
        n:usize,
        mut abn:[(usize,usize);n],
    }
    // let mut v:Vec<_> = abn.iter().enumerate().map(|(j,&x)|(j,x.1-x.0)).collect();
    // v.sort_by_key(|k|k.1);
    abn.sort();
    abn.sort_by_key(|k|k.1);
    // println!("{:?}",v);
    let mut l = 0;
    // for &(j,last_start) in &v {
    //            println!("l:{} last_start:{}",l,last_start);
    //     println!("{:?}",abn[j]);
 
    //     l += abn[j].0;
    //     if l > abn[j].1 {
    //         println!("No");
    //         return;
    //     }
    for &(a,b) in &abn{
        l += a;
        if l > b {
            println!("No");
            return;
        }
    }
    // }
    println!("Yes");
}