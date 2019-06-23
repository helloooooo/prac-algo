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
fn main() {
    input! {
        x:i64,
        y:i64,
        z:i64,
        k:i64,
        an:[i64;x],
        bn:[i64;y],
        cn:[i64;z],
    }
    let mut cn = cn;
    cn.sort();
    cn.reverse();
    let mut abn = vec![];
    for i in 0..x as usize {
        for j in 0..y as usize {
            abn.push(an[i] + bn[j]);
        }
    }
    abn.sort();
    abn.reverse();
    let rep = std::cmp::min(k, x * y);
    let mut ans = vec![];
    for j in 0..rep as usize {
        for k in 0..z as usize {
            ans.push(abn[j] + cn[k]);
        }
    }
    ans.sort();
    ans.reverse();
    for j in 0..k as usize {
        println!("{}", ans[j]);
    }
}
