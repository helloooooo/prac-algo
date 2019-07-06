pub mod modular {
    const m: i64 = 1000000007;
 
    #[derive(debug, clone, copy, default, partialord, ord, partialeq, eq)]
    pub struct mod(i64);
 
    impl ::std::fmt::display for mod {
        fn fmt(&self, f: &mut ::std::fmt::formatter) -> ::std::fmt::result {
            write!(f, "{}", self.0)
        }
    }
 
    impl mod {
        pub fn new(v: i64) -> mod {
            mod(v % m)
        }
 
        pub fn pow(self, mut r: i64) -> mod {
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
 
        // this requires m is prime
        pub fn recip(self) -> mod {
            self.pow(m - 2)
        }
    }
 
    use std::ops::*;
 
    impl<t: into<mod>> add<t> for mod {
        type output = mod;
        fn add(self, rhs: t) -> self::output {
            mod::new(self.0 + rhs.into().0)
        }
    }
    impl<t: into<mod>> addassign<t> for mod {
        fn add_assign(&mut self, rhs: t) {
            *self = *self + rhs;
        }
    }
 
    impl<t: into<mod>> sub<t> for mod {
        type output = mod;
        fn sub(self, rhs: t) -> self::output {
            mod::new(self.0 - rhs.into().0 + m)
        }
    }
    impl<t: into<mod>> subassign<t> for mod {
        fn sub_assign(&mut self, rhs: t) {
            *self = *self - rhs;
        }
    }
 
    impl<t: into<mod>> mul<t> for mod {
        type output = mod;
        fn mul(self, rhs: t) -> self::output {
            mod::new(self.0 * rhs.into().0)
        }
    }
    impl<t: into<mod>> mulassign<t> for mod {
        fn mul_assign(&mut self, rhs: t) {
            *self = *self * rhs;
        }
    }
 
    impl<t: into<mod>> div<t> for mod {
        type output = mod;
        fn div(self, rhs: t) -> self::output {
            self * rhs.into().recip()
        }
    }
    impl<t: into<mod>> divassign<t> for mod {
        fn div_assign(&mut self, rhs: t) {
            *self = *self / rhs;
        }
    }
 
    impl neg for mod {
        type output = mod;
        fn neg(self) -> self::output {
            mod(0) - self
        }
    }
 
    impl<t: ::std::convert::into<i64>> ::std::convert::from<t> for mod {
        fn from(v: t) -> self {
            mod::new(v.into())
        }
    }
}
