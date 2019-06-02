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
fn main() {
    input! {
        n:i64,
        an:[i64;n],
    }
    let map = an.iter().fold(HashMap::new(), |mut map, x| {
        let y = map.get(&x).map(|&i| i + 1).unwrap_or(1);
        map.insert(x, y);
        map
    });
    let master = map
        .iter()
        .filter(|&(k, v)| *v == 2)
        .map(|(k, v)| *v)
        .collect::<Vec<i64>>();
    if master.len() as i64 != n / 2 {
        println!("0");
        return;
    }
    let mut ans = master.iter().fold(1, |y, x| (y * x) % (1e9 as i64 + 7));
    if n % 2 != 0 && an.iter().filter(|&x| *x == 0).count() != 1 {
        println!("0");
        return;
    }
    println!("{}", ans);
}
