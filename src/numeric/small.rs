use std::ops::Rem;

use quickcheck::Arbitrary;

/// In this case, "`x` is small" means "`x * x * x` does not overflow".
/// Also, `Small` is always non-zero.
/// It is primarily intended for testing various math functions.
#[derive(Clone, Copy, Debug)]
pub struct Small<T: Rem<T>>(pub T);

macro_rules! impl_small {
    ($T:ident % $m:expr) => {
        impl Arbitrary for Small<$T> {
            fn arbitrary(g: &mut quickcheck::Gen) -> Self {
                let value = $T::arbitrary(g) % $m;
                Self(if value != (0 as $T) { value } else { (1 as $T) })
            }
        }
    };
}

impl_small!(u8 % 0x4);
impl_small!(u16 % 0x10);
impl_small!(u32 % 0x100);
impl_small!(u64 % 0x10000);

impl_small!(i8 % 0x4);
impl_small!(i16 % 0x10);
impl_small!(i32 % 0x100);
impl_small!(i64 % 0x10000);

impl_small!(usize % 0x100);
impl_small!(isize % 0x100);

impl_small!(f32 % 10.0);
impl_small!(f64 % 100.0);
