use bytemuck::Pod;

pub mod collections;
pub mod pod;
pub mod types;

/// Trait to represent types with zero-copy deserialization.
pub trait ZeroCopy
where
    Self: Pod,
{
    #[inline]
    fn load(data: &[u8]) -> &Self {
        bytemuck::from_bytes(&data[..std::mem::size_of::<Self>()])
    }

    #[inline]
    fn load_mut(data: &mut [u8]) -> &mut Self {
        bytemuck::from_bytes_mut(&mut data[..std::mem::size_of::<Self>()])
    }
}
