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
const MOD: i64 = 1e9 as i64 + 7;
use std::collections::HashMap;
fn main() {
    input! {
        n:i64,
    }
    let mut map = HashMap::new();
    for j in 2..n + 1 {
        for k in 2..j + 1 {
            if !is_prime(k) {
                continue;
            }
            if j % k != 0 {
                continue;
            }
            let mut count = 0;
            let mut tmp = j;
            while tmp % k == 0 {
                count += 1;
                tmp /= k;
            }
            let target = map.entry(k).or_insert(0);
            *target += count;
        }
    }
    let mut ans = 1;
    for (_, v) in map {
        ans *= v + 1;
        ans %= MOD;
    }
    println!("{}", ans);
}
fn is_prime(n: i64) -> bool {
    let mut res = true;
    for i in 2..n {
        if i * i > n {
            break;
        }
        if n % i == 0 {
            res = false;
            break;
        }
    }
    res
}
