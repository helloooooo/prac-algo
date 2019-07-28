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
fn main(){
    input!{
        n:usize,
        an:[i64;n],
    }
    let mut ans = vec![0;n];
    let mut count = 0;
    for i in (1..n+1).rev() {
        let mut sum = 0;
        let mut index = i;
        while index <= n {
            sum += ans[index-1];
            index += i;
        }
        if sum % 2 != an[i-1] {
            ans[i-1] = 1;
        }
    }
    let mut sub = vec![];
    for i in 0..ans.len() {
        if ans[i] != 0 {
            sub.push(i+1);
        }
    }
    println!("{}",sub.len());
    for i in 0..sub.len(){
        if i > 0 {
            print!(" ");
        }
        print!("{}",sub[i]);
    }
    println!();
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
