use core::mem::size_of;

use crate::error::{Result, TranslationError};

/// Marker trait for types that can be safely cast from a raw pointer.
///
/// # Safety
///
/// Types implementing this trait must guarantee that the cast is safe,
/// i.e., ensuring proper field alignment and absence of padding bytes.
pub unsafe trait Transmute: Sized {
    /// Creates a reference to `Self` from a byte slice.
    #[inline(always)]
    fn transmute(bytes: &[u8]) -> Result<&Self> {
        if bytes.len() != size_of::<Self>() {
            return Err(TranslationError::InvalidLength);
        }

        Ok(unsafe { Self::transmute_unchecked(bytes) })
    }

    /// Creates a reference to `Self` from a byte slice.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `bytes` contains a valid representation of
    /// the implementing type, including proper alignment and length.
    #[inline(always)]
    unsafe fn transmute_unchecked(bytes: &[u8]) -> &Self {
        &*(bytes.as_ptr() as *const Self)
    }

    /// Creates a mutable reference to `Self` from a mutable byte slice.
    #[inline(always)]
    fn transmute_mut(bytes: &mut [u8]) -> Result<&mut Self> {
        if bytes.len() != size_of::<Self>() {
            return Err(TranslationError::InvalidLength);
        }

        Ok(unsafe { Self::transmute_unchecked_mut(bytes) })
    }

    /// Creates a mutable reference to `Self` from a mutable byte slice.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `bytes` contains a valid representation of
    /// the implementing type, including proper alignment and length.
    #[inline(always)]
    unsafe fn transmute_unchecked_mut(bytes: &mut [u8]) -> &mut Self {
        &mut *(bytes.as_mut_ptr() as *mut Self)
    }
}

/// Convert `&[A]` into `&[B]`.
///
/// Since target `B` may have a different size than source `A`, the resulting slice
/// will potentially have a different length than the original slice.
///
/// # Safety
///
/// The caller must ensure that the size of `B` is a multiple of the size of `A`,
/// and that the alignment of `&[A]` is compatible with `&[B]`.
pub unsafe fn cast_slice_unchecked<A: Transmute, B: Transmute>(bytes: &[A]) -> &[B] {
    let len = bytes.len() / size_of::<B>();
    unsafe { core::slice::from_raw_parts(bytes.as_ptr() as *const B, len) }
}

/// Convert `&mut [A]` into `&mut [B]`.
///
/// Since target `B` may have a different size than source `A`, the resulting slice
/// will potentially have a different length than the original slice.
///
/// # Safety
///
/// The caller must ensure that the size of `B` is a multiple of the size of `A`,
/// and that the alignment of `&[A]` is compatible with `&[B]`.
pub unsafe fn cast_slice_unchecked_mut<A: Transmute, B: Transmute>(bytes: &mut [A]) -> &mut [B] {
    let len = bytes.len() / size_of::<B>();
    unsafe { core::slice::from_raw_parts_mut(bytes.as_mut_ptr() as *mut B, len) }
}

// Convenience macro to implement the `Transmute` trait for common types.
macro_rules! imp_transmute {
    ( $($type:ty),* ) => {
        $(
            unsafe impl Transmute for $type {}
        )*
    };
}

// Implement the `Transmute` trait for common numeric types.
imp_transmute!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64, usize, isize);
