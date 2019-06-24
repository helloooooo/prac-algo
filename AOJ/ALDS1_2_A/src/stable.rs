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
        mut an:[usize;n],
    }
    let mut ans = 0;
    for j in 0..n {
        let mut min_index = j;
        for k in j..n {
            if an[k] < an[min_index] {
                min_index = k;
            }
        }
        if min_index != j {
            an.swap(min_index,j);
        }        
    }
    for j in 0..n-1{
        print!("{} ",an[j]);
    }
    println!("{}",an[n-1]);
    println!("{}",ans);
}



fn selection_sort(an:&Vec<usize>) -> Vec<usize> {
    let mut an = an.clone();
    let n = an.len();
    for j in 0..n {
        let mut min_index = j;
        for k in j..n {
            if an[k] < an[min_index] {
                min_index = k;
            }
        }
        if min_index != j {
            an.swap(min_index,j);
        }      
    }
    an
}
fn bubble_sort(an:&Vec<usize>) -> Vec<usize> {
    let mut flag = true;
    let mut an = an.clone();
    let n = an.len();
    while flag {
        flag = false;
        for j in (1..n).rev() {
            if an[j] < an[j-1] {
                an.swap(j-1,j);
                flag = true;
            }
        }
    }
    an
}
fn is_stable(an:&Vec<usize>,origin:&Vec<usize>,n:usize){
    for j in 0..n {
        for k in j+1..n {
            for a in 0..n {
                for b in a+1..n {
                    if origin[j] -== 
                }
            }
        }
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
