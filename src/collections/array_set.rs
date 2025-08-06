use core::{cmp::Ordering, ops::Deref, ptr::copy};

use crate::transmute::{cast_slice_unchecked, cast_slice_unchecked_mut, Transmute};

/// Macro to implement the read-only interface for an array set type.
macro_rules! readonly_impl {
    ( $name:tt ) => {
        impl<'a, V> $name<'a, V>
        where
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
            pub const fn is_empty(&self) -> bool {
                self.len() == 0
            }

            #[inline(always)]
            pub const fn is_full(&self) -> bool {
                self.len() == self.values.len()
            }

            #[inline(always)]
            pub const fn len(&self) -> usize {
                *self.length as usize
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
                let mut end = (*self.length - 1) as usize;

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

        impl<'a, V> Deref for $name<'a, V>
        where
            V: Clone + Copy + Default + Ord + PartialOrd + Transmute,
        {
            type Target = [V];

            fn deref(&self) -> &Self::Target {
                &self.values[..self.len()]
            }
        }
    };
}

/// Macro to implement an array set type.
macro_rules! prefix_array_set {
    ( $name:tt, $prefix_type:tt ) => {
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
        pub struct $name<'a, V>
        where
            V: Clone + Copy + Default + PartialOrd + Transmute,
        {
            /// Number of elements in the array
            ///
            /// This number reflects the used positions.
            length: &'a $prefix_type,

            /// Array of nodes to store the tree.
            values: &'a [V],
        }

        impl<'a, V> $name<'a, V>
        where
            V: Copy + Clone + Default + PartialOrd + Transmute,
        {
            /// Loads a sorted array from its byte representation.
            ///
            /// # Safety
            ///
            /// This method does not check the length of the byte slice nor its
            /// alignment. The caller must ensure that the byte slice contains a
            /// valid representation.
            pub unsafe fn from_bytes_unchecked(bytes: &'a [u8]) -> Self {
                let (length, values) = bytes.split_at(size_of::<$prefix_type>());
                Self {
                    length: <$prefix_type>::transmute_unchecked(length),
                    values: cast_slice_unchecked(values),
                }
            }
        }
    };
}

prefix_array_set!(U8ArraySet, u8);
prefix_array_set!(U16ArraySet, u16);
prefix_array_set!(U32ArraySet, u32);
prefix_array_set!(U64ArraySet, u64);

readonly_impl!(U8ArraySet);
readonly_impl!(U16ArraySet);
readonly_impl!(U32ArraySet);
readonly_impl!(U64ArraySet);

/// Macro to implement a mutable array set type.
macro_rules! prefix_array_set {
    ( $name:tt, $prefix_type:tt ) => {
        /// A mutable set-like type that stores elements in a sorted array.
        pub struct $name<'a, V>
        where
            V: Default + Copy + Clone + Ord + Transmute,
        {
            /// Number of elements in the array
            ///
            /// This number reflects the used positions.
            length: &'a mut $prefix_type,

            /// Array of nodes to store the tree.
            values: &'a mut [V],
        }

        impl<'a, V> $name<'a, V>
        where
            V: Default + Copy + Clone + Ord + Transmute,
        {
            /// Loads a sorted array from its byte representation.
            ///
            /// # Safety
            ///
            /// This method does not check the length of the byte slice nor its
            /// alignment. The caller must ensure that the byte slice contains a
            /// valid representation.
            pub unsafe fn from_bytes_unchecked_mut(bytes: &'a mut [u8]) -> Self {
                let (length, values) = bytes.split_at_mut(size_of::<$prefix_type>());
                Self {
                    length: <$prefix_type>::transmute_unchecked_mut(length),
                    values: cast_slice_unchecked_mut(values),
                }
            }

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
                    *self.length += 1;
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
                    *self.length -= 1;
                    return Some(value);
                }

                None
            }
        }
    };
}

prefix_array_set!(U8ArraySetMut, u8);
prefix_array_set!(U16ArraySetMut, u16);
prefix_array_set!(U32ArraySetMut, u32);
prefix_array_set!(U64ArraySetMut, u64);

readonly_impl!(U8ArraySetMut);
readonly_impl!(U16ArraySetMut);
readonly_impl!(U32ArraySetMut);
readonly_impl!(U64ArraySetMut);

#[cfg(test)]
mod tests {
    use core::slice::from_raw_parts_mut;

    use super::*;

    #[test]
    fn test_insert() {
        let mut array = [0u64; size_of::<u64>() * 3];
        // Ensure that `bytes` has 8-byte alignment.
        let bytes = unsafe { from_raw_parts_mut(array.as_mut_ptr() as *mut u8, 18) };
        let mut set = unsafe { U64ArraySetMut::<u8>::from_bytes_unchecked_mut(bytes) };

        set.insert(10);
        set.insert(1);
        set.insert(2);
        set.insert(7);
        set.insert(4);

        let set = unsafe { U64ArraySet::<u8>::from_bytes_unchecked(bytes) };
        assert_eq!(set.len(), 5);
        assert_eq!(&*set, &[1, 2, 4, 7, 10]);

        assert!(set.get(&1).is_some());
    }

    #[test]
    fn test_remove() {
        let mut bytes = [0; size_of::<u8>() + 10 * size_of::<u8>()];
        let mut set = unsafe { U8ArraySetMut::<u8>::from_bytes_unchecked_mut(&mut bytes) };

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
        let mut set = unsafe { U8ArraySetMut::<u8>::from_bytes_unchecked_mut(&mut bytes) };

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
