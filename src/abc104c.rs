use std::cmp::min;
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

fn main(){
    input!{
        d:usize,
        g:i64,
        v:[(i64,i64);d],
    }
    let mut ans = std::i64::MAX;
    for bit in (0..1<<d) {
        let mut count = 0;
        let mut value_count = 0;
        for j in 0..d as i64 {
            if bit & (1 << j) != 0 {
                count += v[j as  usize].0;
                value_count += (j+1) * 100 * v[j as usize].0 + v[j as usize].1;
            }
        }
        if value_count >= g {
            ans = min(ans,count);
            continue;
        }
        let best = (0,0);
        for j in (0..d as i64).rev() {
            if bit & (1<<j) != 0 {
                continue;
            }

            if value_count + (j+1) * 100 * v[j as usize].0 >= g {
                let u = (g- value_count + (j + 1) * 100 - 1)/((j + 1) * 100);
                count += u;
                break;
            }
            count += v[j as usize].0;
            value_count += (j + 1) * 100 * v[j as usize].0;
        }
        ans = min(ans,count);
    }
    println!("{}",ans);
}