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
        an:[usize;n],
    }
    shell_sort(&an, n);
}
fn insertion_sort(an:&mut Vec<usize>,g:usize) -> usize  {
    let mut count = 0;
    let n = an.len();
    for j in g..n{
        let mut l = j-g;
        while 0 < l  && an[l-1] > an[l] {
            an.swap(l-1,l);
            l -= 1;
            count += 1;
        }
    }
    count
}
fn shell_sort(an:&Vec<usize>,n:usize) {
    let mut m = 0;
    let mut g_v = vec![];
    let mut h = 1;
    while h < n / 9{
        h = h*3 + 1;
    }
    let mut v = an.clone();
    let mut count = 0;
    for &g in &g_v {
        count = insertion_sort(&mut v,g);
    }
    println!("{}",g_v.len());
    for j in 0..g_v.len()-1 {
        print!("{} ",g_v[j]);
    }
    println!("{}",g_v[g_v.len()-1]);
    println!("{}",count);
    for &x in &v {
        println!("{}",x);
    }
}

fn is_prime(x:i64) -> bool {
    if x == 2 {return true;}
    if x < 2 || x % 2 == 0 {return false;}
    let mut j = 3;
    while j <= (x as f64).sqrt() as i64 {
        if x % j == 0 {
            return false;
        }
        j += 2;
    }
    true
}


#[test]
fn shell_test(){
    let n = 5;
    let an = vec![5,1,4,3,2];
    shell_sort(&an, n);
    assert!(true);
}