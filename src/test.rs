#![allow(unused_imports)]
 
use std::cmp;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt;
use std::io::{self, BufWriter, Read, Write};
use std::ops::{Add, Div, Mul, Sub};
use std::str::{self, FromStr};
 
 
fn solve(f: Vec<Vec<u8>>, p: Vec<Vec<i64>>) -> i64 {
    let f = f.into_iter().map(decode).collect::<Vec<_>>();
    (0b1..0b10000000000)
        .map(|m| {
            f.iter()
                .cloned()
                .enumerate()
                .map(|(i, b)| p[i][(m & b).count_ones() as usize])
                .sum()
        })
        .max()
        .unwrap()
}
 
 
fn decode(b: Vec<u8>) -> u32 {
    b.into_iter()
        .rev()
        .enumerate()
        .map(|(i, d)| (d as u32) << i)
        .sum()
}
 
 
fn main() {
    let stdin = io::stdin();
    let mut sc = Scanner::new(stdin.lock().bytes());
    let n = sc.next();
    println!("{}", solve(sc.mat(n, 10), sc.mat(n, 11)));
}
 
 
struct Scanner<R: Read> {
    buf: Vec<u8>,
    bytes: io::Bytes<R>,
}
 
#[allow(dead_code)]
impl<R: Read> Scanner<R> {
    fn new(bytes: io::Bytes<R>) -> Self {
        Scanner {
            buf: Vec::with_capacity(1024),
            bytes: bytes,
        }
    }
 
    fn next<T: FromStr>(&mut self) -> T
    where
        <T as FromStr>::Err: fmt::Debug,
    {
        let buf = &mut self.buf;
        buf.clear();
        self.bytes
            .by_ref()
            .map(Result::unwrap)
            .skip_while(|&c| c == ' ' as u8 || c == '\n' as u8)
            .take_while(|&c| !(c == ' ' as u8 || c == '\n' as u8))
            .foreach(|c| buf.push(c));
        str::from_utf8(&buf).unwrap().parse().unwrap()
    }
 
    fn vec<T: FromStr>(&mut self, n: usize) -> Vec<T>
    where
        <T as FromStr>::Err: fmt::Debug,
    {
        (0..n).map(|_| self.next()).collect()
    }
 
    fn pairs<T1: FromStr, T2: FromStr>(&mut self, n: usize) -> Vec<(T1, T2)>
    where
        <T1 as FromStr>::Err: fmt::Debug,
        <T2 as FromStr>::Err: fmt::Debug,
    {
        (0..n).map(|_| (self.next(), self.next())).collect()
    }
 
    fn trios<T1: FromStr, T2: FromStr, T3: FromStr>(&mut self, n: usize) -> Vec<(T1, T2, T3)>
    where
        <T1 as FromStr>::Err: fmt::Debug,
        <T2 as FromStr>::Err: fmt::Debug,
        <T3 as FromStr>::Err: fmt::Debug,
    {
        (0..n)
            .map(|_| (self.next(), self.next(), self.next()))
            .collect()
    }
 
    fn mat<T: FromStr>(&mut self, m: usize, n: usize) -> Vec<Vec<T>>
    where
        <T as FromStr>::Err: fmt::Debug,
    {
        (0..m).map(|_| self.vec(n)).collect()
    }
 
    fn strings_as_mat<T, F: FnMut(u8) -> T>(&mut self, h: usize, mut f: F) -> Vec<Vec<T>> {
        (0..h)
            .map(|_| {
                self.bytes
                    .by_ref()
                    .map(Result::unwrap)
                    .skip_while(|&c| c == ' ' as u8 || c == '\n' as u8)
                    .take_while(|&c| !(c == ' ' as u8 || c == '\n' as u8))
                    .map(&mut f)
                    .collect()
            })
            .collect()
    }
}
 
 
trait Foreach {
    type Item;
    fn foreach<F: FnMut(Self::Item)>(self, f: F);
}
 
impl<T, I: Iterator<Item = T>> Foreach for I {
    type Item = T;
 
    fn foreach<F: FnMut(T)>(self, mut f: F) {
        self.fold((), move |(), x| f(x));
    }
}