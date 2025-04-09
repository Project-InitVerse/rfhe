use super::{CastFrom, CastInto, Numeric, SignedNumeric, UnsignedInteger};
use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};


pub trait SignedInteger:
    SignedNumeric
    + Neg<Output = Self>
    + Add<Self, Output = Self>
    + AddAssign<Self>
    + Div<Self, Output = Self>
    + DivAssign<Self>
    + Mul<Self, Output = Self>
    + MulAssign<Self>
    + Rem<Self, Output = Self>
    + RemAssign<Self>
    + Sub<Self, Output = Self>
    + SubAssign<Self>
    + BitAnd<Self, Output = Self>
    + BitAndAssign<Self>
    + BitOr<Self, Output = Self>
    + BitOrAssign<Self>
    + BitXor<Self, Output = Self>
    + BitXorAssign<Self>
    + Not<Output = Self>
    + Shl<usize, Output = Self>
    + ShlAssign<usize>
    + Shr<usize, Output = Self>
    + ShrAssign<usize>
    + CastFrom<f64>
    + CastInto<f64>
{

    type Unsigned: UnsignedInteger<Signed = Self> + CastFrom<Self>;


    fn into_unsigned(self) -> Self::Unsigned;


    fn to_bits_string(&self, block_length: usize) -> String;


    #[must_use]
    fn wrapping_abs(self) -> Self;
}

macro_rules! implement {
    ($Type: tt, $UnsignedType:ty, $bits:expr) => {
        impl Numeric for $Type {
            const BITS: usize = $bits;
            const ZERO: Self = 0;
            const ONE: Self = 1;
            const TWO: Self = 2;
            const MAX: Self = <$Type>::MAX;
        }

        impl SignedNumeric for $Type {
            type NumericUnsignedType = $UnsignedType;
        }

        impl SignedInteger for $Type {
            type Unsigned = $UnsignedType;
            #[inline]
            fn into_unsigned(self) -> Self::Unsigned {
                Self::Unsigned::cast_from(self)
            }
            fn to_bits_string(&self, break_every: usize) -> String {
                let mut strn = match <$Type as Numeric>::BITS {
                    8 => format!("{:08b}", self),
                    16 => format!("{:016b}", self),
                    32 => format!("{:032b}", self),
                    64 => format!("{:064b}", self),
                    128 => format!("{:0128b}", self),
                    _ => unreachable!(),
                };
                for i in (1..(<$Type as Numeric>::BITS / break_every)).rev() {
                    strn.insert(i * break_every, ' ');
                }
                strn
            }

            #[inline]
            fn wrapping_abs(self) -> Self {
                self.wrapping_abs()
            }
        }
    };
}

implement!(i8, u8, 8);
implement!(i16, u16, 16);
implement!(i32, u32, 32);
implement!(i64, u64, 64);
implement!(i128, u128, 128);
implement!(isize, usize, isize::BITS as usize);
