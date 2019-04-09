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
use std::cmp::{max,min};
fn main(){
    input!{
        w:i64,
        h:i64,
        n:i64,
        xyan:[[i64;3];n],
    }
    let mut sizes = vec![0,w,0,h];
    for j in 0..n as usize{
        let x = xyan[j as usize][0];
        let y = xyan[j as usize][1];
        let a = xyan[j as usize][2];
        if a == 1 {
            sizes[0] = max(sizes[0],x);
        } else if a ==2 {
            sizes[1] = min(sizes[1],x);
        } else if a == 3 {
            sizes[2] = max(sizes[2],y);
        } else if a ==4 {
            sizes[3] = min(sizes[3],y);
        }
    }
    let width = sizes[1] - sizes[0];
    let width = if width < 0  { 0 } else { width};
    let heigt = sizes[3] - sizes[2];
    let heigt = if heigt < 0 { 0 } else {heigt};
    println!("{}",  width * heigt);
}