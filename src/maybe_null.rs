use crate::{error::TranslationError, nullable::Nullable, transmute::Transmute};

/// A "pod-enabled" type that can be used as an `Option<T>` without
/// requiring extra space to indicate if the value is `Some` or `None`.
///
/// This can be used when a specific value of `T` indicates that its
/// value is `None`.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MaybeNull<T: Nullable + Copy>(T);

unsafe impl<T: Nullable + Copy> Transmute for MaybeNull<T> {}

impl<T: Nullable + Copy> Default for MaybeNull<T> {
    fn default() -> Self {
        Self(T::NONE)
    }
}

impl<T: Nullable + Copy> MaybeNull<T> {
    /// Returns the contained value as an `Option`.
    #[inline(always)]
    pub fn get(self) -> Option<T> {
        if self.0.is_none() {
            None
        } else {
            Some(self.0)
        }
    }

    /// Returns a reference to the contained value as an `Option`.
    #[inline(always)]
    pub fn as_ref(&self) -> Option<&T> {
        if self.0.is_none() {
            None
        } else {
            Some(&self.0)
        }
    }

    /// Returns the contained value as a mutable `Option`.
    #[inline(always)]
    pub fn as_mut(&mut self) -> Option<&mut T> {
        if self.0.is_none() {
            None
        } else {
            Some(&mut self.0)
        }
    }
}

impl<T: Nullable + Copy> From<T> for MaybeNull<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T: Nullable + Copy> TryFrom<Option<T>> for MaybeNull<T> {
    type Error = TranslationError;

    fn try_from(value: Option<T>) -> Result<Self, Self::Error> {
        match value {
            Some(value) if value.is_none() => Err(TranslationError::InvalidValue),
            Some(value) => Ok(Self(value)),
            None => Ok(Self(T::NONE)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {
        let maybe_null = MaybeNull::<[u8; 32]>::default();
        assert_eq!(maybe_null.get(), None);

        let bytes = [1u8; 32];
        let maybe_null = MaybeNull::from(bytes);
        assert!(maybe_null.get().is_some());
        assert!(maybe_null.get().unwrap() == bytes);

        let bytes = [2u8; 32];
        let maybe_null = MaybeNull::<[u8; 32]>::transmute(&bytes).unwrap();
        assert!(maybe_null.get().is_some());
        assert!(maybe_null.get().unwrap() == bytes);
    }
}
