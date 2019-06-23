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
        s:chars,
        t:i64,
    }
    let ans: (i64, i64, i64) = s.iter().fold((0, 0, 0), |y, c| match *c {
        'L' => (y.0 - 1, y.1, y.2),
        'R' => (y.0 + 1, y.1, y.2),
        'U' => (y.0, y.1 + 1, y.2),
        'D' => (y.0, y.1 - 1, y.2),
        '?' => (y.0, y.1, y.2 + 1),
        _ => unreachable!(),
    });
    let ans = if t == 1 {
        ans.0.abs() + ans.1.abs() + ans.2
    } else {
        std::cmp::max(s.len() as i64 % 2, ans.0.abs() + (ans.1).abs() - ans.2)
    };
    println!("{}", ans);
}
