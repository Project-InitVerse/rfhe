use super::{CastFrom, CastInto, Numeric, SignedInteger, UnsignedNumeric};
use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

/// A trait shared by all the unsigned integer types.
pub trait UnsignedInteger:
    UnsignedNumeric
    + Ord
    + Eq
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
    + CastFrom<Self::Signed>
    + CastFrom<f64>
    + CastInto<f64>
    + CastFrom<u128>
    + CastInto<u128>
    + std::fmt::Binary
    + From<bool>
{
    /// The signed type of the same precision.
    type Signed: SignedInteger<Unsigned = Self> + CastFrom<Self>;
    /// Return the leading zeros of the value.
    #[must_use]
    fn leading_zeros(self) -> u32;
    /// Compute an addition, modulo the max of the type.
    #[must_use]
    fn wrapping_add(self, other: Self) -> Self;
    /// Compute a subtraction, modulo the max of the type.
    #[must_use]
    fn wrapping_sub(self, other: Self) -> Self;
    /// Compute an addition, modulo a custom modulus.
    #[must_use]
    fn wrapping_add_custom_mod(self, other: Self, custom_modulus: Self) -> Self;
    /// Compute a subtraction, modulo a custom modulus.
    #[must_use]
    fn wrapping_sub_custom_mod(self, other: Self, custom_modulus: Self) -> Self;
    /// Compute a division, modulo the max of the type.
    #[must_use]
    fn wrapping_div(self, other: Self) -> Self;
    /// Compute a multiplication, modulo the max of the type.
    #[must_use]
    fn wrapping_mul(self, other: Self) -> Self;
    /// Compute a multiplication, modulo a custom modulus.
    #[must_use]
    fn wrapping_mul_custom_mod(self, other: Self, custom_modulus: Self) -> Self;
    /// Compute the remainder, modulo the max of the type.
    #[must_use]
    fn wrapping_rem(self, other: Self) -> Self;
    /// Compute a negation, modulo the max of the type.
    #[must_use]
    fn wrapping_neg(self) -> Self;
    /// Compute a negation, modulo the max of the type.
    #[must_use]
    fn wrapping_neg_custom_mod(self, custom_modulus: Self) -> Self;
    /// Compute an exponentiation, modulo the max of the type.
    #[must_use]
    fn wrapping_pow(self, exp: u32) -> Self;
    /// Panic free shift-left operation.
    #[must_use]
    fn wrapping_shl(self, rhs: u32) -> Self;
    /// Panic free shift-right operation.
    #[must_use]
    fn wrapping_shr(self, rhs: u32) -> Self;
    #[must_use]
    fn overflowing_add(self, rhs: Self) -> (Self, bool);
    #[must_use]
    fn is_power_of_two(self) -> bool;
    #[must_use]
    fn next_power_of_two(self) -> Self;
    #[must_use]
    fn ilog2(self) -> u32;
    #[must_use]
    fn ceil_ilog2(self) -> u32 {
        // ilog2 returns the rounded down log2
        self.ilog2() + u32::from(!self.is_power_of_two())
    }
    /// Integer division rounding up.
    #[must_use]
    fn div_ceil(self, divisor: Self) -> Self;
    /// Return the casting of the current value to the signed type of the same size.
    fn into_signed(self) -> Self::Signed;
    /// Return a bit representation of the integer, where blocks of length `block_length` are
    /// separated by whitespaces to increase the readability.
    fn to_bits_string(&self, block_length: usize) -> String;
}

macro_rules! implement {
    ($Type: tt, $SignedType:ty, $bits:expr) => {
        impl Numeric for $Type {
            const BITS: usize = $bits;
            const ZERO: Self = 0;
            const ONE: Self = 1;
            const TWO: Self = 2;
            const MAX: Self = <$Type>::MAX;
        }

        impl UnsignedNumeric for $Type {
            type NumericSignedType = $SignedType;
        }

        impl UnsignedInteger for $Type {
            type Signed = $SignedType;
            #[inline]
            fn into_signed(self) -> Self::Signed {
                Self::Signed::cast_from(self)
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
            fn leading_zeros(self) -> u32 {
                self.leading_zeros()
            }
            #[inline]
            fn wrapping_add(self, other: Self) -> Self {
                self.wrapping_add(other)
            }
            #[inline]
            fn wrapping_sub(self, other: Self) -> Self {
                self.wrapping_sub(other)
            }
            #[inline]
            fn wrapping_add_custom_mod(self, other: Self, custom_modulus: Self) -> Self {
                if Self::BITS <= 64 {
                    let self_u128: u128 = self.cast_into();
                    let other_u128: u128 = other.cast_into();
                    let custom_modulus_u128: u128 = custom_modulus.cast_into();
                    self_u128
                        .wrapping_add(other_u128)
                        .wrapping_rem(custom_modulus_u128)
                        .cast_into()
                } else {
                    if custom_modulus.is_power_of_two() {
                        return self.wrapping_add(other).wrapping_rem(custom_modulus);
                    }
                    todo!("wrapping_add_custom_mod is not yet implemented for non power of two moduli wider than u64")
                }
            }
            #[inline]
            fn wrapping_sub_custom_mod(self, other: Self, custom_modulus: Self) -> Self {
                if Self::BITS <= 64 {
                    let self_u128: u128 = self.cast_into();
                    let other_u128: u128 = other.cast_into();
                    let custom_modulus_u128: u128 = custom_modulus.cast_into();
                    self_u128
                        .wrapping_add(custom_modulus_u128)
                        .wrapping_sub(other_u128)
                        .wrapping_rem(custom_modulus_u128)
                        .cast_into()
                } else {
                    if custom_modulus.is_power_of_two() {
                        return self.wrapping_sub(other).wrapping_rem(custom_modulus);
                    }
                    todo!("wrapping_sub_custom_mod is not yet implemented for non power of two moduli wider than u64")
                }
            }
            #[inline]
            fn wrapping_div(self, other: Self) -> Self {
                self.wrapping_div(other)
            }
            #[inline]
            fn wrapping_mul(self, other: Self) -> Self {
                self.wrapping_mul(other)
            }
            #[inline]
            fn wrapping_mul_custom_mod(self, other: Self, custom_modulus: Self) -> Self {
                if Self::BITS <= 64 {
                    let self_u128: u128 = self.cast_into();
                    let other_u128: u128 = other.cast_into();
                    let custom_modulus_u128: u128 = custom_modulus.cast_into();
                    self_u128
                        .wrapping_mul(other_u128)
                        .wrapping_rem(custom_modulus_u128)
                        .cast_into()
                } else {
                    todo!("wrapping_mul_custom_mod is not yet implemented for types wider than u64")
                }
            }
            #[inline]
            fn wrapping_rem(self, other: Self) -> Self {
                self.wrapping_rem(other)
            }
            #[inline]
            fn wrapping_neg(self) -> Self {
                self.wrapping_neg()
            }
            #[inline]
            fn wrapping_neg_custom_mod(self, custom_modulus: Self) -> Self {
                // Custom modulus applied by wrapping_sub
                Self::ZERO.wrapping_sub_custom_mod(self, custom_modulus)
            }
            #[inline]
            fn wrapping_shl(self, rhs: u32) -> Self {
                self.wrapping_shl(rhs)
            }
            #[inline]
            fn wrapping_shr(self, rhs: u32) -> Self {
                self.wrapping_shr(rhs)
            }
            #[inline]
            fn wrapping_pow(self, exp: u32) -> Self {
                self.wrapping_pow(exp)
            }
            #[inline]
            fn overflowing_add(self, rhs: Self) -> (Self, bool) {
                self.overflowing_add(rhs)
            }
            #[inline]
            fn is_power_of_two(self) -> bool {
                self.is_power_of_two()
            }
            #[inline]
            fn next_power_of_two(self) -> Self {
                self.next_power_of_two()
            }
            #[inline]
            fn ilog2(self) -> u32 {
                self.ilog2()
            }
            #[inline]
            fn div_ceil(self, divisor: Self) -> Self {
                self.div_ceil(divisor)
            }
        }
    };
}

implement!(u8, i8, 8);
implement!(u16, i16, 16);
implement!(u32, i32, 32);
implement!(u64, i64, 64);
implement!(u128, i128, 128);
implement!(usize, isize, usize::BITS as usize);

