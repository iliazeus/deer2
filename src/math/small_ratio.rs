use std::cmp::Ordering;
use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use super::*;

/// Primarily used for testing certain algebraic properties.
/// Not optimized for speed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SmallRatio64(i64, i64);

#[allow(non_camel_case_types)]
pub type r64 = SmallRatio64;

impl SmallRatio64 {
    pub fn new(num: i64, den: i64) -> Self {
        assert!(den > 0, "denominator must be positive");
        let gcd = gcd_i64(num.abs(), den);
        Self(num / gcd, den / gcd)
    }

    pub fn num(&self) -> i64 {
        self.0
    }

    pub fn den(&self) -> i64 {
        self.1
    }
}

impl Num for SmallRatio64 {
    const EPS: Self = Self(1, 100);
    const PI: Self = Self(355, 113);

    fn from_usize(x: usize) -> Self {
        Self(x as i64, 1)
    }

    fn abs(self) -> Self {
        Self(self.0.abs(), self.1)
    }

    fn sqrt(self) -> Self {
        unimplemented!()
    }

    fn sin(self) -> Self {
        unimplemented!()
    }

    fn cos(self) -> Self {
        unimplemented!()
    }
}

#[cfg(test)]
use quickcheck::Arbitrary;

#[cfg(test)]
impl Arbitrary for SmallRatio64 {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let smalls: &[i64] = &[1, -1, 2, -2, 3, -3, 5, -5, 7, -7];

        let num = *g.choose(smalls).unwrap();
        let den = (*g.choose(smalls).unwrap()).abs();

        SmallRatio64::new(num, den)
    }
}

impl Display for SmallRatio64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let SmallRatio64(num, den) = self;
        write!(f, "{num} / {den}")
    }
}

impl Zero for SmallRatio64 {
    const ZERO: Self = Self(0, 1);
}

impl One for SmallRatio64 {
    const ONE: Self = Self(1, 1);
}

impl PartialOrd for SmallRatio64 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        i64::partial_cmp(&(self.num() * self.den()), &(other.num() * other.den()))
    }
}

impl Ord for SmallRatio64 {
    fn cmp(&self, other: &Self) -> Ordering {
        i64::cmp(&(self.num() * self.den()), &(other.num() * other.den()))
    }
}

impl Neg for SmallRatio64 {
    type Output = SmallRatio64;
    fn neg(self) -> Self::Output {
        SmallRatio64(-self.num(), self.den())
    }
}

impl Add<SmallRatio64> for SmallRatio64 {
    type Output = SmallRatio64;
    fn add(self, rhs: SmallRatio64) -> Self::Output {
        let den_gcd = gcd_i64(self.den(), rhs.den());

        SmallRatio64::new(
            self.num() * (rhs.den() / den_gcd) + rhs.num() * (self.den() / den_gcd),
            (self.den() / den_gcd) * rhs.den(),
        )
    }
}

impl Sub<SmallRatio64> for SmallRatio64 {
    type Output = SmallRatio64;
    fn sub(self, rhs: SmallRatio64) -> Self::Output {
        let den_gcd = gcd_i64(self.den(), rhs.den());

        SmallRatio64::new(
            self.num() * (rhs.den() / den_gcd) - rhs.num() * (self.den() / den_gcd),
            (self.den() / den_gcd) * rhs.den(),
        )
    }
}

impl Mul<SmallRatio64> for SmallRatio64 {
    type Output = SmallRatio64;
    fn mul(self, rhs: SmallRatio64) -> Self::Output {
        println!("{self} * {rhs}");

        let gcd_nd = gcd_i64(self.num().abs(), rhs.den());
        let gcd_dn = gcd_i64(self.den(), rhs.num().abs());

        SmallRatio64::new(
            (self.num() / gcd_nd) * (rhs.num() / gcd_dn),
            (self.den() / gcd_dn) * (rhs.den() / gcd_nd),
        )
    }
}

impl SmallRatio64 {
    pub fn inv(self) -> Self {
        assert!(self.num() != 0, "division by zero");
        SmallRatio64(self.den().abs() * self.num().signum(), self.num().abs())
    }
}

impl Div<SmallRatio64> for SmallRatio64 {
    type Output = SmallRatio64;
    fn div(self, rhs: SmallRatio64) -> Self::Output {
        self * rhs.inv()
    }
}

impl AddAssign<SmallRatio64> for SmallRatio64 {
    fn add_assign(&mut self, rhs: SmallRatio64) {
        *self = *self + rhs;
    }
}

impl SubAssign<SmallRatio64> for SmallRatio64 {
    fn sub_assign(&mut self, rhs: SmallRatio64) {
        *self = *self - rhs;
    }
}

impl MulAssign<SmallRatio64> for SmallRatio64 {
    fn mul_assign(&mut self, rhs: SmallRatio64) {
        *self = *self * rhs;
    }
}

impl DivAssign<SmallRatio64> for SmallRatio64 {
    fn div_assign(&mut self, rhs: SmallRatio64) {
        *self = *self / rhs;
    }
}

/// Arguments must be non-negative.
fn gcd_i64(u: i64, v: i64) -> i64 {
    gcd_u64(u as u64, v as u64) as i64
}

/// Adapted from https://en.wikipedia.org/wiki/Binary_GCD_algorithm
#[rustfmt::skip]
fn gcd_u64(mut u: u64, mut v: u64) -> u64 {
    use std::cmp::min;
    use std::mem::swap;

    // Base cases: gcd(n, 0) = gcd(0, n) = n
    if u == 0 {
        return v;
    } else if v == 0 {
        return u;
    }

    // Using identities 2 and 3:
    // gcd(2ⁱ u, 2ʲ v) = 2ᵏ gcd(u, v) with u, v odd and k = min(i, j)
    // 2ᵏ is the greatest power of two that divides both u and v
    let i = u.trailing_zeros();  u >>= i;
    let j = v.trailing_zeros();  v >>= j;
    let k = min(i, j);

    loop {
        // u and v are odd at the start of the loop
        debug_assert!(u % 2 == 1, "u = {} is even", u);
        debug_assert!(v % 2 == 1, "v = {} is even", v);

        // Swap if necessary so u <= v
        if u > v {
            swap(&mut u, &mut v);
        }
        // u and v are still both odd after (potentially) swapping

        // Using identity 4 (gcd(u, v) = gcd(|v-u|, min(u, v))
        v -= u;
        // v is now even, but u is unchanged (and odd)

        // Identity 1: gcd(u, 0) = u
        // The shift by k is necessary to add back the 2ᵏ factor that was removed before the loop
        if v == 0 {
            return u << k;
        }

        // Identity 3: gcd(u, 2ʲ v) = gcd(u, v) (u is known to be odd)
        v >>= v.trailing_zeros();
        // v is now odd again
    }
}
