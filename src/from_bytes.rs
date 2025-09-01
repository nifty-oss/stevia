use crate::error::Result;

/// Marker trait for types that can be created from a byte slice.
///
/// # Safety
///
/// Types implementing this trait must ensure that the byte slice is
/// properly aligned and that the bytes represent a valid instance of the type.
pub unsafe trait FromBytes<T> {
    /// Creates an instance of `Self` from a byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the byte slice does not have the correct length.
    fn from_bytes(bytes: &[u8]) -> Result<T>;

    /// Creates an instance of `Self` from a byte slice.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `bytes` contains a valid representation of
    /// the implementing type.
    unsafe fn from_bytes_unchecked(bytes: &[u8]) -> T;
}

/// Marker trait for types that can be created from a mutable byte slice.
///
/// # Safety
///
/// Types implementing this trait must ensure that the byte slice is
/// properly aligned and that the bytes represent a valid instance of the type.
///
/// Caution should be taken when the type offers interior mutability, given that
/// the source byte slice is mutable.
pub unsafe trait FromBytesMut<T> {
    /// Creates an instance of `Self` from a byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the byte slice does not have the correct length.
    fn from_bytes_mut(bytes: &mut [u8]) -> Result<T>;

    /// Creates an instance of `Self` from a byte slice.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `bytes` contains a valid representation of
    /// the implementing type.
    fn from_bytes_unchecked_mut(bytes: &mut [u8]) -> T;
}
