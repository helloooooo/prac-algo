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
const INF:i64 = 1e10 as i64;
struct DP {
    target:Vec<usize>,
    costs:Vec<i64>,
}
impl DP {
    fn new(n:usize,target:Vec<usize>) -> DP {
        DP {
            target,
            costs:vec![0;n],
        }
    }
    fn calc(&mut self,n:i64) -> i64 {
        for 
    }
}
fn main(){
    input!{
        n:i64,
        m:i64,
        mat:[[i64;2];m],
    }
    let mut field = [[bool;n];n];
    for arr in &mat {
        field[arr[0][0]-1][arr[0][1]-1] = true;
        field[arr[0][1]-1][arr[0][0]-1] = true;
    }
    println!("{:?}",field);
}
