use crate::serde::uleb128::v32;

pub trait ByteSized {
    fn byte_size(&self) -> usize;
}

macro_rules! impl_bytesized {
    ($t:ty) => {
        impl ByteSized for $t {
            fn byte_size(&self) -> usize {
                std::mem::size_of::<Self>()
            }
        }
    };
    ($t:ty, $($tt:ty), +) => {
        impl_bytesized!($t);
        impl_bytesized!($($tt), +);
    }
}

impl_bytesized!(bool, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);

impl<const N: usize, T: ByteSized> ByteSized for [T; N] {
    fn byte_size(&self) -> usize {
        self.iter().map(|n| T::byte_size(n)).sum()
    }
}

impl ByteSized for str {
    fn byte_size(&self) -> usize {
        if self.len() == 0 {
            1
        } else {
            1 + v32(self.len() as _).byte_size() + self.len()
        }
    }
}

impl ByteSized for String {
    fn byte_size(&self) -> usize {
        str::byte_size(self)
    }
}
