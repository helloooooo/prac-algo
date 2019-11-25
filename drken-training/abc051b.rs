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
use std::collections::{HashMap,HashSet};
use std::cmp::{max,min};

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



pub mod modular {
  const M: i64 = 1000000007;

  #[derive(Debug, Clone, Copy, Default, PartialOrd, Ord, PartialEq, Eq)]
  pub struct Mod(i64);

  impl ::std::fmt::Display for Mod {
      fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
          write!(f, "{}", self.0)
      }
  }

  impl Mod {
      pub fn new(v: i64) -> Mod {
          Mod(v % M)
      }

      pub fn pow(self, mut r: i64) -> Mod {
          let mut k = self;
          let mut ret = 1.into();

          while r > 0 {
              if r % 2 != 0 {
                  ret = ret * k;
              }
              r /= 2;
              k = k * k;
          }

          ret
      }

      // This requires M is prime
      pub fn recip(self) -> Mod {
          self.pow(M - 2)
      }
  }

  use std::ops::*;

  impl<T: Into<Mod>> Add<T> for Mod {
      type Output = Mod;
      fn add(self, rhs: T) -> Self::Output {
          Mod::new(self.0 + rhs.into().0)
      }
  }
  impl<T: Into<Mod>> AddAssign<T> for Mod {
      fn add_assign(&mut self, rhs: T) {
          *self = *self + rhs;
      }
  }

  impl<T: Into<Mod>> Sub<T> for Mod {
      type Output = Mod;
      fn sub(self, rhs: T) -> Self::Output {
          Mod::new(self.0 - rhs.into().0 + M)
      }
  }
  impl<T: Into<Mod>> SubAssign<T> for Mod {
      fn sub_assign(&mut self, rhs: T) {
          *self = *self - rhs;
      }
  }

  impl<T: Into<Mod>> Mul<T> for Mod {
      type Output = Mod;
      fn mul(self, rhs: T) -> Self::Output {
          Mod::new(self.0 * rhs.into().0)
      }
  }
  impl<T: Into<Mod>> MulAssign<T> for Mod {
      fn mul_assign(&mut self, rhs: T) {
          *self = *self * rhs;
      }
  }

  impl<T: Into<Mod>> Div<T> for Mod {
      type Output = Mod;
      fn div(self, rhs: T) -> Self::Output {
          self * rhs.into().recip()
      }
  }
  impl<T: Into<Mod>> DivAssign<T> for Mod {
      fn div_assign(&mut self, rhs: T) {
          *self = *self / rhs;
      }
  }

  impl Neg for Mod {
      type Output = Mod;
      fn neg(self) -> Self::Output {
          Mod(0) - self
      }
  }

  impl<T: ::std::convert::Into<i64>> ::std::convert::From<T> for Mod {
      fn from(v: T) -> Self {
          Mod::new(v.into())
      }
  }
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

fn main(){
  input!{
    k:i64,
    s:i64,
  }
  let mut ans = 0;
  for x in 0..k+1 {
    for y in 0..k+1 {
      let z = s - x - y;
      if 0 <= z && z <= k {  
        ans += 1;
      }
    }
  }
  println!("{}",ans);
}
