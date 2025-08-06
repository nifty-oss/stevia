use crate::transmute::Transmute;

/// Trait for types that can be `None`.
///
/// This trait is used to indicate that a type can be `None` according to a
/// specific value. When a type `T` implements this trait, it can be used in
/// combination with `MaybeNull` to represent optional values can have the
/// same representation as `T`.
pub trait Nullable: PartialEq + Transmute {
    /// Value that represents `None` for the type.
    const NONE: Self;

    /// Indicates whether the value is `None` or not.
    #[inline(always)]
    fn is_none(&self) -> bool {
        self == &Self::NONE
    }

    /// Indicates whether the value is `Some` value of type `T` or not.
    #[inline(always)]
    fn is_some(&self) -> bool {
        !self.is_none()
    }
}
