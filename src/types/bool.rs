use crate::transmute::Transmute;

/// A type representing a boolean value that can be transmuted
/// from an `u8` representation.
///
/// This type represents a boolean value where `0` is `false`
/// and any non-zero value is `true`.
#[repr(transparent)]
pub struct Bool(u8);

unsafe impl Transmute for Bool {}

impl Bool {
    /// Returns the inner value as a `bool`.
    #[inline(always)]
    pub const fn value(&self) -> bool {
        self.0 != 0
    }
}

impl From<bool> for Bool {
    #[inline(always)]
    fn from(value: bool) -> Self {
        Bool(value as u8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value() {
        let b = Bool::from(true);
        assert!(b.value());

        let bytes = &[0];
        let b = Bool::transmute(bytes).unwrap();
        assert!(!b.value());

        let bytes = &[5];
        let b = Bool::transmute(bytes).unwrap();
        assert!(b.value());
    }
}
