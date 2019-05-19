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
fn main(){
    input!{
        n:i64,
        k:i64
    }
    let mut i = 1;
    let mut div = vec![];
    while i <= n {
        if i >= k {
            break;
        }
        div.push(calc(i,&k));
        i += 1;
    }
    let max_v= div.iter().max().unwrap();
    let mut sum:f64= div.iter().fold(0,|y,x| {
        if max_v - x == 0  {
            y + 1
        } else {
            (max_v-x).abs()*2 + y
        }
    }) as f64;

    let master = calc_2(&max_v);
        if (div.len() as i64 )< n {
        sum += (n - div.len() as i64) as f64* master; 
    }
    let ans = sum/(master*n as f64);

    println!("{:?}",ans);
}
fn calc(n:i64,k:&i64) -> i64{
    let mut sub = n.clone();
    let mut ans = 0;
    while sub <= *k {
        sub *= 2;
        ans += 1;
    }
    ans
}
fn calc_2(n:&i64) -> f64 {
    let mut t = 2;
    for _ in 0..*n-1 {
        t *= 2;
    }
    t as f64
}