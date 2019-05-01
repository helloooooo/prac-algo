
use std::cmp::{min,max};
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
// 再帰関数での実装
const MOVES: [(i32, i32); 4]  = [(1,0),(0,1),(-1,0),(0,-1)];
fn main(){
    input!{
        h:usize,
        w:usize,
        field:[chars;h],
    }
    let mut s_index = (0,0);
    let mut g_index = (0,0);
    for j in 0..h {
        for k in 0..w {
            if field[j][k] == 's' {
                s_index = (j,k);
            } else if field[j][k] == 'g' {
                g_index = (j,k);
            }
        }
    }
    let mut solver = DFS{
        h:h,
        w:w,
        field:field,
        dist:vec![vec![false;w];h],
    };
    solver.search(s_index);
    println!("{}", if solver.dist[g_index.0][g_index.1] { "Yes"} else {"No"})
}

struct DFS {
    h:usize,
    w:usize,
    field:Vec<Vec<char>>,
    dist:Vec<Vec<bool>>
}
impl DFS {
    fn search(&mut self,point:(usize,usize)){
        if self.h <= point.0 || self.w <= point.1 
            || self.field[point.0][point.1] == '#' {
                return;
        }
        if self.dist[point.0][point.1] {return;} else { self.dist[point.0][point.1] = true;}
        for &mv in &MOVES{
            let after = (point.0 as i32 + mv.0 , point.1 as i32 + mv.1);
            if after.0 < 0 || after.1 < 0 {
                continue;
            }
            let after = (after.0 as usize, after.1 as usize);
            self.search(after);
        }
    }
}