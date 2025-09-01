use core::{cmp::Ordering, mem::size_of, ops::Deref, ptr::copy};

use crate::{
    error::{Result, TranslationError},
    from_bytes::{FromBytes, FromBytesMut},
    transmute::{cast_slice_unchecked, cast_slice_unchecked_mut, Transmute},
};

pub trait Prefix: Transmute {
    fn as_usize(&self) -> usize;

    fn decrement(&mut self);

    fn increment(&mut self);
}

// Convenience macro to implement the `Prefix` trait for common numeric types.
macro_rules! impl_prefix {
    ( $($type:ty),* ) => {
        $(
            impl Prefix for $type {
                #[inline(always)]
                fn as_usize(&self) -> usize {
                    *self as usize
                }

                #[inline(always)]
                fn decrement(&mut self) {
                    *self -= 1;
                }

                #[inline(always)]
                fn increment(&mut self) {
                    *self += 1;
                }
            }
        )*
    };
}

// Implement the `Transmute` trait for common numeric types.
impl_prefix!(u8, u16, u32, u64, u128, usize);

/// A set-like type that stores elements in a sorted array.
///
/// It requires that the elements implement the `Ord` trait. It is a logic error
/// for a value to be modified in such a way that the value's order, as determined by
/// the [`Ord`] trait, or its equality, as determined by the [`Eq`] trait, changes while
/// it is in the set.
///
/// The behavior resulting from either logic error is not specified, but will
/// be encapsulated to the `ArraySet` that observed the logic error and not
/// result in undefined behavior. This could include panics, incorrect results,
/// aborts, memory leaks, and non-termination.
pub struct ArraySet<'a, P, V>
where
    P: Prefix,
    V: Clone + Copy + Default + PartialOrd + Transmute,
{
    /// Number of elements in the array
    ///
    /// This number reflects the used positions.
    length: &'a P,

    /// Array of nodes to store the tree.
    values: &'a [V],
}

/// Macro to implement the read-only interface for an array set type.
macro_rules! readonly_impl {
    ( $name:tt ) => {
        impl<'a, P, V> $name<'a, P, V>
        where
            P: Prefix,
            V: Clone + Copy + Default + Ord + PartialOrd + Transmute,
        {
            /// Returns true if the set contains a value.
            ///
            /// The value may be any borrowed form of the set's value type, but `Ord` on the
            /// borrowed form must match those for the value type.
            pub fn contains(&self, value: &V) -> bool {
                self.get(value).is_some()
            }

            /// Returns a reference to the value in the set, if any, that is equal to the
            /// given value.
            ///
            /// The value may be any borrowed form of the set's value type, but `Ord` on the
            /// borrowed form must match those for the value type.
            pub fn get(&self, value: &V) -> Option<&V> {
                if let (Some(index), _) = self.index(value) {
                    Some(&self.values[index])
                } else {
                    None
                }
            }

            #[inline(always)]
            pub fn is_empty(&self) -> bool {
                self.len() == 0
            }

            #[inline(always)]
            pub fn is_full(&self) -> bool {
                self.len() == self.values.len()
            }

            #[inline(always)]
            pub fn len(&self) -> usize {
                self.length.as_usize()
            }

            /// Returns the index of the value in the array.
            ///
            /// The return value determines if the value is already in the array
            /// or it is a new value that should be inserted. The first return value
            /// will be present if the value is already in the array; in this case
            /// the returned value is the index of the value in the array. The second
            /// return value will be present if the value is not in the array; in this
            /// case the returned value is the index where the value should be
            /// inserted.
            fn index(&self, value: &V) -> (Option<usize>, Option<usize>) {
                if self.is_empty() {
                    // array is empty, the value should be inserted at start
                    // of the values array
                    return (None, Some(0));
                }

                let mut start = 0;
                let mut end = self.length.as_usize() - 1;

                while start <= end {
                    let middle = start + (end.saturating_sub(start) / 2);

                    match value.cmp(&self.values[middle]) {
                        // if we are already at the start of the array, there are no
                        // more elements to check
                        Ordering::Less if end == start => break,

                        // value might be in the first half of the array
                        Ordering::Less => end = middle.saturating_sub(1),

                        // value might be in the second half of the array
                        Ordering::Greater => start = middle.saturating_add(1),

                        // found the value in the array
                        Ordering::Equal => {
                            return (Some(middle), None);
                        }
                    }
                }

                // value is not in the array, return the index where it should
                // be inserted
                (None, Some(start))
            }
        }

        impl<'a, P, V> Deref for $name<'a, P, V>
        where
            P: Prefix,
            V: Clone + Copy + Default + Ord + PartialOrd + Transmute,
        {
            type Target = [V];

            fn deref(&self) -> &Self::Target {
                &self.values[..self.len()]
            }
        }
    };
}

/// A mutable set-like type that stores elements in a sorted array.
pub struct ArraySetMut<'a, P, V>
where
    P: Prefix,
    V: Default + Copy + Clone + Ord + Transmute,
{
    /// Number of elements in the array
    ///
    /// This number reflects the used positions.
    length: &'a mut P,

    /// Array of nodes to store the tree.
    values: &'a mut [V],
}

impl<P, V> ArraySetMut<'_, P, V>
where
    P: Prefix,
    V: Default + Copy + Clone + Ord + Transmute,
{
    /// Returns a mutable reference to the value in the set, if any, that is equal to the
    /// given value.
    ///
    /// The value may be any borrowed form of the set's value type, but `Ord` on the
    /// borrowed form must match those for the value type.
    ///
    /// It is a logic error for a value to be modified in such a way that the value's order,
    /// as determined by the [`Ord`] trait, or its equality, as determined by the [`Eq`] trait,
    /// changes while it is in the set.
    pub fn get_mut(&mut self, value: &V) -> Option<&mut V> {
        if let (Some(index), _) = self.index(value) {
            Some(&mut self.values[index])
        } else {
            None
        }
    }

    /// Adds a value to the set.
    ///
    /// Returns whether the value was newly inserted. That is:
    ///
    /// - If the set did not previously contain this value, `true` is returned.
    /// - If the set already contained this value, `false` is returned,
    ///   and the set is not modified: original value is not replaced,
    ///   and the value passed as argument is dropped.
    /// - If the set is full, `false` is returned.
    pub fn insert(&mut self, value: V) -> bool {
        // does not attempt to insert if the array is full
        if self.is_full() {
            return false;
        }

        if let (_, Some(index)) = self.index(&value) {
            unsafe {
                let ptr = self.values.as_mut_ptr();
                let src_ptr = ptr.add(index);
                let dest_ptr = ptr.add(index + 1);
                // move the bytes to create space for the new value
                copy(src_ptr, dest_ptr, self.values.len() - (index + 1));
            }
            // insert the new value
            self.values[index] = value;
            self.length.increment();
            return true;
        }

        false
    }

    /// Removes a value from the set and returns whether the value was present in the set.
    ///
    /// The value may be any borrowed form of the set's value type, but
    /// [`Ord`] on the borrowed form *must* match those for the value type.
    pub fn remove(&mut self, value: &V) -> bool {
        self.take(value).is_some()
    }

    /// Removes and returns the value in the set, if any, that is equal to the given one.
    ///
    /// The value may be any borrowed form of the set's value type, but
    /// [`Ord`] on the borrowed form *must* match those for the value type.
    pub fn take(&mut self, value: &V) -> Option<V> {
        // does not attempt to remove if the array is empty
        if self.is_empty() {
            return None;
        }

        if let (Some(index), _) = self.index(value) {
            let value = *self.values.get(index).unwrap();

            // only need to copy bytes around if the element being removed
            // is not the last element in the array
            if index < self.len() - 1 {
                unsafe {
                    let ptr = self.values.as_mut_ptr();
                    let src_ptr = ptr.add(index + 1);
                    let dest_ptr = ptr.add(index);
                    // move the bytes after the value being removed
                    copy(src_ptr, dest_ptr, self.values.len() - (index + 1));
                }
            }
            self.length.decrement();
            return Some(value);
        }

        None
    }
}

unsafe impl<'a, P, V> FromBytes<'a> for ArraySet<'a, P, V>
where
    P: Prefix,
    V: Default + Copy + Clone + Ord + Transmute,
{
    /// Loads a sorted array from its byte representation.
    fn from_bytes(bytes: &'a [u8]) -> Result<Self> {
        if bytes.len() < size_of::<P>() {
            return Err(TranslationError::InvalidLength);
        }

        if align_of_val(bytes) != align_of::<Self>() {
            return Err(TranslationError::InvalidLength);
        }

        Ok(unsafe { Self::from_bytes_unchecked(bytes) })
    }

    /// Loads a sorted array from its byte representation.
    ///
    /// # Safety
    ///
    /// This method does not check the length of the byte slice nor its
    /// alignment. The caller must ensure that the byte slice contains a
    /// valid representation.
    unsafe fn from_bytes_unchecked(bytes: &'a [u8]) -> Self {
        let (length, values) = bytes.split_at(size_of::<P>());
        Self {
            length: P::transmute_unchecked(length),
            values: cast_slice_unchecked(values),
        }
    }
}

unsafe impl<'a, P, V> FromBytesMut<'a> for ArraySetMut<'a, P, V>
where
    P: Prefix,
    V: Default + Copy + Clone + Ord + Transmute,
{
    /// Loads a sorted array from its byte representation.
    fn from_bytes_mut(bytes: &'a mut [u8]) -> Result<Self> {
        if bytes.len() < size_of::<P>() {
            return Err(TranslationError::InvalidLength);
        }

        if align_of_val(bytes) != align_of::<Self>() {
            return Err(TranslationError::InvalidLength);
        }

        Ok(unsafe { Self::from_bytes_unchecked_mut(bytes) })
    }

    /// Loads a sorted array from its byte representation.
    ///
    /// # Safety
    ///
    /// This method does not check the length of the byte slice nor its
    /// alignment. The caller must ensure that the byte slice contains a
    /// valid representation.
    unsafe fn from_bytes_unchecked_mut(bytes: &'a mut [u8]) -> Self {
        let (length, values) = bytes.split_at_mut(size_of::<P>());
        Self {
            length: P::transmute_unchecked_mut(length),
            values: cast_slice_unchecked_mut(values),
        }
    }
}

readonly_impl!(ArraySet);
readonly_impl!(ArraySetMut);

#[cfg(test)]
mod tests {
    use core::slice::from_raw_parts_mut;

    use super::*;

    #[test]
    fn test_insert() {
        let mut array = [0u64; size_of::<u64>() * 3];
        // Ensure that `bytes` has 8-byte alignment.
        let bytes = unsafe { from_raw_parts_mut(array.as_mut_ptr() as *mut u8, 18) };
        let mut set = unsafe { ArraySetMut::<u64, u8>::from_bytes_unchecked_mut(bytes) };

        set.insert(10);
        set.insert(1);
        set.insert(2);
        set.insert(7);
        set.insert(4);

        let set = unsafe { ArraySet::<u64, u8>::from_bytes_unchecked(bytes) };
        assert_eq!(set.len(), 5);
        assert_eq!(&*set, &[1, 2, 4, 7, 10]);

        assert!(set.get(&1).is_some());
    }

    #[test]
    fn test_remove() {
        let mut bytes = [0; size_of::<u8>() + 10 * size_of::<u8>()];
        let mut set = unsafe { ArraySetMut::<u8, u8>::from_bytes_unchecked_mut(&mut bytes) };

        set.insert(1);
        set.insert(10);
        set.insert(2);
        set.insert(7);
        set.insert(4);

        assert_eq!(&*set, &[1, 2, 4, 7, 10]);

        set.remove(&2);
        assert_eq!(set.len(), 4);
        assert_eq!(&*set, &[1, 4, 7, 10]);

        set.remove(&10);
        assert_eq!(set.len(), 3);
        assert_eq!(&*set, &[1, 4, 7]);

        set.remove(&4);
        assert_eq!(set.len(), 2);
        assert_eq!(&*set, &[1, 7]);

        set.remove(&1);
        assert_eq!(set.len(), 1);
        assert_eq!(&*set, &[7]);
    }

    #[test]
    fn test_get() {
        let mut bytes = [0; size_of::<u8>() + 10 * size_of::<u8>()];
        let mut set = unsafe { ArraySetMut::<u8, u8>::from_bytes_unchecked_mut(&mut bytes) };

        set.insert(1);
        set.insert(10);
        set.insert(7);
        set.insert(2);
        set.insert(4);

        assert_eq!(&*set, &[1, 2, 4, 7, 10]);

        assert!(set.get(&10).is_some());
        assert!(set.get_mut(&10).is_some());
    }
}
