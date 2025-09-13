//! Primitive types that support transmutation and nullability.

mod bool;
mod str;

pub use bool::Bool;
pub use str::Str;

use crate::{nullable::Nullable, transmute::Transmute};

// Implement `Transmute` and `Nullable` for unsigned integer
// array types.
macro_rules! transmute_unsigned_array_type {
    ( $type:tt ) => {
        unsafe impl<const N: usize> Transmute for [$type; N] {}

        impl<const N: usize> Nullable for [$type; N] {
            const NONE: Self = [0; N];
        }
    };
}

transmute_unsigned_array_type!(u8);
transmute_unsigned_array_type!(u16);
transmute_unsigned_array_type!(u32);
transmute_unsigned_array_type!(u64);

/// A trait for converting a value to `usize`.
///
/// This trait is implemented for primitive integer types,
/// which can be used as generic parameters where a numeric
/// value is needed to represent sizes or counts.
pub trait ToUsize {
    fn to_usize(&self) -> usize;
}

/// A trait representing a unit value for a type.
pub trait Unit {
    const UNIT: Self;
}

// Implement `ToUsize` and `Unit` for unsigned integer types.
macro_rules! as_usize_and_unit_for_unsigned_type {
    ( $type:tt ) => {
        impl ToUsize for $type {
            fn to_usize(&self) -> usize {
                *self as usize
            }
        }

        impl Unit for $type {
            const UNIT: Self = 1;
        }
    };
}

as_usize_and_unit_for_unsigned_type!(u8);
as_usize_and_unit_for_unsigned_type!(u16);
as_usize_and_unit_for_unsigned_type!(u32);
as_usize_and_unit_for_unsigned_type!(u64);

#[cfg(test)]
mod tests {
    use core::slice::from_raw_parts;

    use super::*;

    #[test]
    fn test_array() {
        // u8
        let bytes: [u8; 3] = [1, 2, 3];

        let values = <[u8; 3]>::transmute(&bytes).unwrap();
        assert_eq!(values, &bytes);

        // u16
        let array: [u16; 3] = [1, 2, 3];
        let bytes = unsafe { from_raw_parts(array.as_ptr() as *const u8, 6) };

        let values = <[u16; 3]>::transmute(bytes).unwrap();
        assert_eq!(values, &array);

        // u32
        let array: [u32; 3] = [1, 2, 3];
        let bytes = unsafe { from_raw_parts(array.as_ptr() as *const u8, 12) };

        let values = <[u32; 3]>::transmute(bytes).unwrap();
        assert_eq!(values, &array);

        // u64
        let array: [u64; 3] = [1, 2, 3];
        let bytes = unsafe { from_raw_parts(array.as_ptr() as *const u8, 24) };

        let values = <[u64; 3]>::transmute(bytes).unwrap();
        assert_eq!(values, &array);
    }
}
