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
struct UnioFind{
    par : Vec<usize>,
    rank : Vec<usize>,
}
impl UnioFind{
    fn new(n:usize) -> UnioFind{
        let mut vec = vec![0;n];
        for i in 0..n {
            vec[i] = i;
        }
        UnioFind{
            par :vec,
            rank :vec![0;n],
        }
    }
    fn find(&mut self,x:usize) -> usize {
        if x == self.par[x]{
            x
        } else {
            let par = self.par[x];
            let res = self.find(par);
            self.par[x] = res;
            res
        }
    }
    fn same(&mut self, a:usize,b:usize) -> bool {
        self.find(a) == self.find(b)
    }
    fn unite(&mut self,a:usize, b:usize){
        let apar = self.find(a);
        let bpar = self.find(b);
        if self.rank[apar] > self.rank[bpar]{
            self.par[bpar] = apar;
        } else {
            self.par[apar] = bpar;
            if self.rank[apar] == self.rank[bpar] {
                self.rank[bpar] +=1;
            }
        }
    }
}

fn main(){
    input!{
        n:usize,
        q:usize,
        que:[(usize,usize,usize);q]
    }
    let mut union_find = UnioFind::new(n);
    for q in &queries {
        match q.0 {
            0 => union_find.unite(q.1,q.2),
            _ => println("{}", if uf.same(q.1,q.2) {"Yes"} else {"No"})
        }
    }
}
