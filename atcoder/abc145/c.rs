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
use std::collections::HashSet;
use std::cmp::{max,min};
fn main(){
  input!{
    n:usize,
    xyn:[[f64;2];n],
  }
  let mut ans = 0.;
  let ret = enumlation(n as u32);
  for i in 0..ret.len() {
    for j in 0..(n -1) {
      let from = ret[i][j]-1;
      let to = ret[i][j+1]-1;
      let x = (xyn[from as usize][0]-xyn[to as usize][0])*(xyn[from as usize][0]-xyn[to as usize][0]);
      let y = (xyn[from as usize][1]-xyn[to as usize][1])*(xyn[from as usize][1]-xyn[to as usize][1]);
      let calc = (x + y).sqrt();
      ans += calc;
    }
  }
  println!("{:.10}",ans/ret.len() as f64);
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

type Pi = Vec<u32>;

fn calc_k(pi:&Pi, i:u32, n:u32) -> u32 {

    // pi(i) + 1 .. n
    let g0: HashSet<u32> = (pi[i as usize - 1] + 1.. n + 2).collect();
    // pi(1) .. pi(i-1)
    let g1: HashSet<u32> = pi[0 .. (i - 1) as usize].to_vec().into_iter().collect();    

    let diff = &g0 - &g1;
    match diff.into_iter().min() {
        Some(k) => k,
        _ => 0
    }   
}

fn enumlation(n: u32) -> Vec<Pi> {
    let mut pi : Pi = (1..n+1).collect();
    let mut i = n - 1;  
    let mut result :  Vec<Vec<u32>> = Vec::new();
    let mut k = calc_k(&pi, i , n);

    result.push(pi.clone());

    // k == n + 1, i == 1
    while k != n + 1 || i != 1  {
        if k <= n {
            pi[i as usize - 1] = k;
            if i == n {
                result.push(pi.clone());
            }
            if i < n  {
                pi[i as usize] = 0;
                i = i + 1;
            }
        }
        if k == n + 1 {
            i = i - 1;
        }
        k = calc_k(&pi, i , n);
    }    
    result   
}
