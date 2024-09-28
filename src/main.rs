trait BetterPartialEq<U> {
    fn better_partial_eq(&self, other: &U) -> Option<bool>;

    fn better_partial_ne(&self, other: &U) -> Option<bool> {
        self.better_partial_eq(other).map(|x| !x)
    }
}

trait BetterEq<U>: BetterPartialEq<U> {
    fn better_eq(&self, other: &U) -> bool {
        self.better_partial_eq(other).unwrap()
    }

    fn better_ne(&self, other: &U) -> bool {
        self.better_partial_ne(other).unwrap()
    }
}

macro_rules! intbeq {
    ($u:ty, $i:ty) => {
        impl BetterPartialEq<$i> for $u {
            fn better_partial_eq(&self, other: &$i) -> Option<bool> {
                Some(if *other < 0 {
                    false
                } else {
                    *self == *other as $u
                })
            }
        }

        impl BetterPartialEq<$u> for $i {
            fn better_partial_eq(&self, other: &$u) -> Option<bool> {
                Some(if *self < 0 {
                    false
                } else {
                    *self as $u == *other
                })
            }
        }

        impl BetterEq<$u> for $i {}
        impl BetterEq<$i> for $u {}
    }
}

intbeq!(u8, i8);
intbeq!(u8, i16);
intbeq!(u8, i32);
intbeq!(u8, i64);
intbeq!(u8, isize);

intbeq!(u16, i8);
intbeq!(u16, i16);
intbeq!(u16, i32);
intbeq!(u16, i64);
intbeq!(u16, isize);

intbeq!(u32, i8);
intbeq!(u32, i16);
intbeq!(u32, i32);
intbeq!(u32, i64);
intbeq!(u32, isize);

intbeq!(u64, i8);
intbeq!(u64, i16);
intbeq!(u64, i32);
intbeq!(u64, i64);
intbeq!(u64, isize);

intbeq!(usize, i8);
intbeq!(usize, i16);
intbeq!(usize, i32);
intbeq!(usize, i64);
intbeq!(usize, isize);

macro_rules! beq {
    (($x:expr) == $y:expr) => {
        $x.better_partial_eq(&$y).unwrap_or(false)
    };
    (($x:expr) != $y:expr) => {
        $x.better_partial_ne(&$y).unwrap_or(false)
    }
}

fn main() {
    assert!(beq!((2u8) != -1i8));
}
