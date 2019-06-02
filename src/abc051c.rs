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
        sx:i64,
        sy:i64,
        gx:i64,
        gy:i64,
    }
    let mut calc = Calc {
        sx,
        sy,
        gx,
        gy,
        go_flag: true,
    };
    calc.first_print();
    calc.second_print();
}

struct Calc {
    sx: i64,
    sy: i64,
    gx: i64,
    gy: i64,
    go_flag: bool,
}

impl Calc {
    fn x_diff(&self) -> i64 {
        (self.sx - self.gx).abs()
    }
    fn y_diff(&self) -> i64 {
        (self.sy - self.gy).abs()
    }

    fn first_print(&mut self) {
        self.go_flag = true;
        for j in 0..2 {
            for j in 0..self.y_diff() {
                if self.go_flag {
                    print!("U");
                } else {
                    print!("D");
                }
            }
            for j in 0..self.x_diff() {
                if self.go_flag {
                    print!("R");
                } else {
                    print!("L");
                }
            }
            self.go_flag = false;
        }
    }

    fn second_print(&mut self) {
        self.go_flag = true;
        for j in 0..2 {
            if self.go_flag {
                print!("L");
            } else {
                print!("R");
            }
            for j in 0..self.y_diff() + 1 {
                if self.go_flag {
                    print!("U");
                } else {
                    print!("D");
                }
            }
            for j in 0..self.x_diff() + 1 {
                if self.go_flag {
                    print!("R");
                } else {
                    print!("L");
                }
            }
            if self.go_flag {
                print!("D");
            } else {
                print!("U")
            }
            self.go_flag = false;
        }
    }
}
