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
        n:i64,
        q:usize,
        s:chars,
        tdn:[(String,String);q],
    }
    let mut s:Vec<(i64,char)> = s.into_iter().enumerate()
        .map(|(i,c)| (i as i64,c))
        .collect();
    let mut count = vec![1;n as usize];
    for (target,dirmove) in tdn {
        for j in  0..n as usize {
            if s[j].1.to_string() == target {
                if dirmove == "L" {
                    if j as i64 - 1 >= 0 {
                        count[j-1] += count[j];
                    } 
                } else {
                    if (j  as i64+ 1)  < n {
                        count[j+1] +=count[j];
                    }
                }
                count[j] = 0;
            }
        }
    }
    let ans:i64 = count.iter().sum();
    println!("{:?}",ans);
}