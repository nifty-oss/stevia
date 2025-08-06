use core::{
    cmp::min,
    str::{from_utf8, from_utf8_unchecked},
};

use crate::{error::TranslationError, transmute::Transmute};

/// A `str` type that can be transmuted from a byte array.
///
/// While the `str` type is backed by a fixed-size byte array,
/// it can represents a `str` of up to `MAX_SIZE` bytes. A null
/// byte (`\0`) is used to indicate the end of the `str` value.
#[repr(C)]
pub struct Str<const MAX_SIZE: usize> {
    /// The bytes of the `str`.
    value: [u8; MAX_SIZE],
}

impl<const MAX_SIZE: usize> Str<MAX_SIZE> {
    /// Returns the bytes of the `str`.
    #[inline(always)]
    pub const fn as_bytes(&self) -> &[u8] {
        &self.value
    }

    /// Tries to convert to a `&str` if it is valid UTF-8.
    ///
    /// Behaves like [`core::from_utf8`].
    #[inline]
    pub fn as_str(&self) -> Result<&str, TranslationError> {
        let end_index = self
            .value
            .iter()
            .position(|&x| x == b'\0')
            .unwrap_or(MAX_SIZE);
        // SAFETY: The `end_index` is guaranteed to be within bounds
        // of `self.value`.
        from_utf8(unsafe { self.value.get_unchecked(..end_index) })
            .map_err(|_| TranslationError::InvalidValue)
    }

    /// Converts to a `&str` without checking if it is valid UTF-8.
    ///
    /// # Safety
    /// The caller must guarantee that the bytes are valid UTF-8. This has the same safety requirements
    /// as [`str::from_utf8_unchecked`].
    #[inline]
    pub unsafe fn as_str_unchecked(&self) -> &str {
        let end_index = self
            .value
            .iter()
            .position(|&x| x == b'\0')
            .unwrap_or(MAX_SIZE);
        unsafe { from_utf8_unchecked(self.value.get_unchecked(..end_index)) }
    }

    /// Copy the content of a slice of `u8`.
    #[inline]
    pub fn copy_from_slice(&mut self, slice: &[u8]) {
        let length = min(slice.len(), MAX_SIZE);
        // SAFETY: The slice is guaranteed to be within bounds of `self.value`.
        unsafe {
            self.value
                .get_unchecked_mut(..length)
                .clone_from_slice(slice.get_unchecked(..length))
        };
        // SAFETY: The slice is guaranteed to be within bounds of `self.value`.
        unsafe {
            self.value.get_unchecked_mut(length..).fill(0);
        }
    }

    /// Copy the content of a `&str`.
    #[inline]
    pub fn copy_from_str(&mut self, string: &str) {
        self.copy_from_slice(string.as_bytes())
    }
}

unsafe impl<const MAX_SIZE: usize> Transmute for Str<MAX_SIZE> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_copy_from_slice() {
        let mut str = Str::<16> { value: [0; 16] };
        str.copy_from_slice(b"Hello, World!");

        assert_eq!(str.as_str().unwrap(), "Hello, World!");
        assert_eq!(str.as_bytes(), b"Hello, World!\0\0\0");
    }

    #[test]
    fn test_transmute() {
        // str: "Hello, World!\0\0\0"
        let bytes: [u8; 16] = [
            72, 101, 108, 108, 111, 44, 32, 87, 111, 114, 108, 100, 33, 0, 0, 0,
        ];
        let str = Str::<16>::transmute(&bytes).unwrap();

        assert_eq!(str.as_str().unwrap(), "Hello, World!");
        assert_eq!(str.as_bytes(), bytes);
    }
}
