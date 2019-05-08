///////////////////////////////////////////// Template /////////////////////////////////////////////
#![allow(non_snake_case, unused)]
use std::collections::*;

//////////////////// missing functions in 1.15.1 ////////////////////
macro_rules! eprint {
	($($t:tt)*) => { {
		use ::std::io::Write;
		let _ = write!(::std::io::stderr(), $($t)*);
	} };
}
macro_rules! eprintln {
	($($t:tt)*) => { {
		use ::std::io::Write;
		let _ = writeln!(::std::io::stderr(), $($t)*);
	} };
}

//////////////////// useful functions ////////////////////
pub trait SetMin {
    fn setmin(&mut self, v: Self) -> bool;
}
impl<T> SetMin for T
where
    T: PartialOrd,
{
    fn setmin(&mut self, v: T) -> bool {
        *self > v && {
            *self = v;
            true
        }
    }
}
pub trait SetMax {
    fn setmax(&mut self, v: Self) -> bool;
}
impl<T> SetMax for T
where
    T: PartialOrd,
{
    fn setmax(&mut self, v: T) -> bool {
        *self < v && {
            *self = v;
            true
        }
    }
}

macro_rules! debug {
	($($v: expr),*) => {
		$(eprint!("{} = {:?} ", stringify!($v), $v);)*
		eprintln!("@ {}:{}", file!(), line!());
	}
}

macro_rules! mat {
	($($e:expr),*) => { Vec::from(vec![$($e),*]) };
	($($e:expr,)*) => { Vec::from(vec![$($e),*]) };
	($e:expr; $d:expr) => { Vec::from(vec![$e; $d]) };
	($e:expr; $d:expr $(; $ds:expr)+) => { Vec::from(vec![mat![$e $(; $ds)*]; $d]) };
}

pub fn readln() -> String {
    let mut line = String::new();
    ::std::io::stdin()
        .read_line(&mut line)
        .unwrap_or_else(|e| panic!("{}", e));
    line
}

macro_rules! read {
	($($t:tt),*; $n:expr) => {{
		let stdin = ::std::io::stdin();
		let ret = ::std::io::BufRead::lines(stdin.lock()).take($n).map(|line| {
			let line = line.unwrap();
			let mut it = line.split_whitespace();
			_read!(it; $($t),*)
		}).collect::<Vec<_>>();
		ret
	}};
	($($t:tt),*) => {{
		let line = readln();
		let mut it = line.split_whitespace();
		_read!(it; $($t),*)
	}};
}

macro_rules! _read {
	($it:ident; [char]) => {
		_read!($it; String).chars().collect::<Vec<_>>()
	};
	($it:ident; [u8]) => {
		Vec::from(_read!($it; String).into_bytes())
	};
	($it:ident; [$t:ty]) => {
		$it.map(|s| s.parse::<$t>().unwrap_or_else(|e| panic!("{}", e))).collect::<Vec<_>>()
	};
	($it:ident; $t:ty) => {
		$it.next().unwrap_or_else(|| panic!("input mismatch")).parse::<$t>().unwrap_or_else(|e| panic!("{}", e))
	};
	($it:ident; $($t:ty),+) => {
		($(_read!($it; $t)),*)
	};
}

pub fn main() {
    let _ = ::std::thread::Builder::new()
        .name("run".to_string())
        .stack_size(32 * 1024 * 1024)
        .spawn(run)
        .unwrap()
        .join();
}
////////////////////////////////////////////////////////////////////////////////////////////////////

pub mod vec {
    use std::ops::*;
    pub type StdVec<T> = ::std::vec::Vec<T>;

    pub trait Idx: Copy {
        fn to_usize(self) -> usize;
    }

    macro_rules! impl_idx {
		($($t:ty),*) => {
			$(
				impl Idx for $t {
					#[inline]
					fn to_usize(self) -> usize {
						self as usize
					}
				}
			)*
		};
	}

    impl_idx!(i32, i64, usize);

    #[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Vec<T>(pub StdVec<T>);

    impl<T> Vec<T> {
        pub fn new() -> Self {
            Vec(StdVec::new())
        }
    }

    impl<T, I: Idx> Index<I> for Vec<T> {
        type Output = T;
        fn index(&self, i: I) -> &T {
            &self.0[i.to_usize()]
        }
    }

    impl<T, I: Idx> IndexMut<I> for Vec<T> {
        fn index_mut(&mut self, i: I) -> &mut T {
            &mut self.0[i.to_usize()]
        }
    }

    pub trait VecUtils<I> {
        fn is_valid(&self, i: I) -> bool;
    }

    impl<T, I: Idx> VecUtils<I> for Vec<T> {
        fn is_valid(&self, i: I) -> bool {
            i.to_usize() < self.len()
        }
    }

    macro_rules! impl_tuple {
		($f:ident) => {
			impl_tuple!($f; A, B; C, D, E, F, G, H, I, J);
		};
		($f:ident; $($i:ident),*; ) => {
			$f!($($i),*);
		};
		($f:ident; $($i:ident),*; $j:ident $(,$k:ident)*) => {
			impl_tuple!($f; $($i),*; );
			impl_tuple!($f; $($i,)* $j; $($k),*);
		};
	}

    macro_rules! impl_vec_tuple {
		(# $i:ident $(,$j:ident)*) => { Vec<impl_vec_tuple!(# $($j),*)> };
		(#) => { T };
		($($i:ident),*) => {
			impl<T, $($i: Idx),*> Index<($($i),*)> for impl_vec_tuple!(# $($i),*) {
				type Output = T;
				#[allow(non_snake_case)]
				fn index(&self, ($($i),*): ($($i),*)) -> &T {
					&self$([$i])*
				}
			}
			impl<T, $($i: Idx),*> IndexMut<($($i),*)> for impl_vec_tuple!(# $($i),*) {
				#[allow(non_snake_case)]
				fn index_mut(&mut self, ($($i),*): ($($i),*)) -> &mut T {
					&mut self$([$i])*
				}
			}
			impl<T, $($i: Idx),*> VecUtils<($($i),*)> for impl_vec_tuple!(# $($i),*) {
				#[allow(non_snake_case)]
				fn is_valid(&self, ($($i),*): ($($i),*)) -> bool {
					let _a = &self;
					$(
						if $i.to_usize() >= _a.len() {
							return false;
						}
						let _a = &_a[$i];
					)*
					true
				}
			}
		};
	}

    impl_tuple!(impl_vec_tuple);

    impl<T> Deref for Vec<T> {
        type Target = StdVec<T>;
        fn deref(&self) -> &StdVec<T> {
            &self.0
        }
    }

    impl<T> DerefMut for Vec<T> {
        fn deref_mut(&mut self) -> &mut StdVec<T> {
            &mut self.0
        }
    }

    impl<T: ::std::fmt::Debug> ::std::fmt::Debug for Vec<T> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
            self.0.fmt(f)
        }
    }

    impl<T: ::std::fmt::Display> ::std::fmt::Display for Vec<T> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
            for (i, a) in self.iter().enumerate() {
                if i > 0 {
                    ' '.fmt(f)?;
                }
                a.fmt(f)?;
            }
            Ok(())
        }
    }

    impl<T> IntoIterator for Vec<T> {
        type Item = T;
        type IntoIter = ::std::vec::IntoIter<T>;
        fn into_iter(self) -> Self::IntoIter {
            self.0.into_iter()
        }
    }

    impl<'a, T> IntoIterator for &'a Vec<T> {
        type Item = &'a T;
        type IntoIter = ::std::slice::Iter<'a, T>;
        fn into_iter(self) -> Self::IntoIter {
            self.0.iter()
        }
    }

    impl<'a, T> IntoIterator for &'a mut Vec<T> {
        type Item = &'a mut T;
        type IntoIter = ::std::slice::IterMut<'a, T>;
        fn into_iter(self) -> Self::IntoIter {
            self.0.iter_mut()
        }
    }

    impl<T> ::std::iter::FromIterator<T> for Vec<T> {
        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
            Vec(StdVec::from_iter(iter))
        }
    }

    impl<S: Into<StdVec<T>>, T> From<S> for Vec<T> {
        fn from(a: S) -> Self {
            Vec(a.into())
        }
    }
}

use vec::*;

const DXY: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn run() {
    let (R, C) = read!(usize, usize);
    let mut s = read!(i32, i32);
    let mut g = read!(i32, i32);
    s.0 -= 1;
    s.1 -= 1;
    g.0 -= 1;
    g.1 -= 1;
    let c = read!([char]; R);
    let mut dist = mat![i32::max_value(); R; C];
    let mut que = BinaryHeap::new();
    println!("{:?}", c);
    dist[s] = 0;
    que.push(s);
    while let Some(p) = que.pop() {
        let d = dist[p];
        for &dir in &DXY {
            let q = (p.0 + dir.0, p.1 + dir.1);
            println!("{} {}", q.0, q.1);
            if c.is_valid(q) && c[q] == '.' && dist[q].setmin(d + 1) {
                que.push(q);
            }
        }
    }
    println!("{}", dist[g]);
}
