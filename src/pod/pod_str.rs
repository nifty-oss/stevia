use crate::ZeroCopy;
use bytemuck::{Pod, Zeroable};
use std::fmt::{Debug, Display};
use std::str;
use std::str::Utf8Error;

/// Struct representing a "pod-enabled" `str`.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct PodStr<const MAX_SIZE: usize> {
    /// The bytes of the string.
    pub value: [u8; MAX_SIZE],
}

impl<const MAX_SIZE: usize> PodStr<MAX_SIZE> {
    pub fn copy_from_slice(&mut self, slice: &[u8]) {
        let length = std::cmp::min(slice.len(), MAX_SIZE);
        self.value[..length].clone_from_slice(&slice[..length]);
        self.value[length..].fill(0);
    }

    /// Copy the content of a `&str` into the pod str.
    pub fn copy_from_str(&mut self, string: &str) {
        self.copy_from_slice(string.as_bytes())
    }

    /// Tries to convert to a `&str` if it is valid UTF-8. Behaves like [`str::from_utf8`].
    pub fn as_str(&self) -> Result<&str, Utf8Error> {
        let end_index = self
            .value
            .iter()
            .position(|&x| x == b'\0')
            .unwrap_or(MAX_SIZE);
        str::from_utf8(&self.value[..end_index])
    }

    /// Converts to a `&str` without checking if it is valid UTF-8.
    ///
    /// # Safety
    /// The caller must guarantee that the bytes are valid UTF-8. This has the same safety requirements
    /// as [`str::from_utf8_unchecked`].
    pub unsafe fn as_str_unchecked(&self) -> &str {
        let end_index = self
            .value
            .iter()
            .position(|&x| x == b'\0')
            .unwrap_or(MAX_SIZE);
        unsafe { str::from_utf8_unchecked(&self.value[..end_index]) }
    }
}

unsafe impl<const MAX_SIZE: usize> Pod for PodStr<MAX_SIZE> {}

unsafe impl<const MAX_SIZE: usize> Zeroable for PodStr<MAX_SIZE> {}

impl<const MAX_SIZE: usize> ZeroCopy for PodStr<MAX_SIZE> {}

impl<const MAX_SIZE: usize> Default for PodStr<MAX_SIZE> {
    fn default() -> Self {
        Self {
            value: [0; MAX_SIZE],
        }
    }
}

impl<const MAX_SIZE: usize> Display for PodStr<MAX_SIZE> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = String::from_utf8_lossy(&self.value);
        formatter.write_str(&str)
    }
}

impl<const MAX_SIZE: usize> From<&str> for PodStr<MAX_SIZE> {
    fn from(s: &str) -> Self {
        let mut value = [0; MAX_SIZE];
        let length = std::cmp::min(s.len(), MAX_SIZE);
        value[..length].clone_from_slice(&s.as_bytes()[..length]);
        Self { value }
    }
}

impl<const MAX_SIZE: usize> From<String> for PodStr<MAX_SIZE> {
    fn from(s: String) -> Self {
        s.as_str().into()
    }
}

#[cfg(test)]
mod tests {
    use bytemuck::bytes_of;

    use crate::{pod::PodStr, ZeroCopy};

    #[test]
    fn test_from() {
        let str = PodStr::<10>::from("str");
        assert_eq!(str.as_str().unwrap(), "str");
    }

    #[test]
    fn test_invalid_bytes() {
        // Invalid utf-8 bytes. The fourth byte has to be 10xxxxxx.
        let invalid_bits: [u8; 4] = [0b1111_0000, 0b1100_0000, 0b1100_0000, 1];
        let mut str = PodStr::<10>::default();
        str.copy_from_slice(&invalid_bits);
        assert!(str.as_str().is_err());
    }

    #[test]
    fn test_copy_from_slice() {
        let mut str = PodStr::<10>::from("empty");
        assert_eq!(str.as_str().unwrap(), "empty");

        // Copy a slice that is equal to the max size.
        str.copy_from_str("emptyempty");
        assert_eq!(str.as_str().unwrap(), "emptyempty");

        // Copy a slice that is smaller than the max size.
        str.copy_from_str("empty");
        assert_eq!(str.as_str().unwrap(), "empty");

        // Copy a slice that is bigger than the max size.
        str.copy_from_str("emptyemptyempty");
        assert_eq!(str.as_str().unwrap(), "emptyempty");
    }

    #[test]
    fn test_load() {
        let str = PodStr::<10>::from("str");
        assert_eq!(str.as_str().unwrap(), "str");

        let bytes = bytes_of(&str);
        let loaded = PodStr::<10>::load(bytes);

        assert_eq!(&str, loaded);
    }
}
