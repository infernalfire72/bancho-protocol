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

impl ByteSized for () {
    fn byte_size(&self) -> usize {
        0
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    // Primitive types
    #[test]
    fn test_byte_size_bool() {
        assert_eq!(true.byte_size(), 1);
        assert_eq!(false.byte_size(), 1);
    }

    #[test]
    fn test_byte_size_u8() {
        assert_eq!(0u8.byte_size(), 1);
    }

    #[test]
    fn test_byte_size_u16() {
        assert_eq!(0u16.byte_size(), 2);
    }

    #[test]
    fn test_byte_size_u32() {
        assert_eq!(0u32.byte_size(), 4);
    }

    #[test]
    fn test_byte_size_u64() {
        assert_eq!(0u64.byte_size(), 8);
    }

    #[test]
    fn test_byte_size_u128() {
        assert_eq!(0u128.byte_size(), 16);
    }

    #[test]
    fn test_byte_size_i8() {
        assert_eq!(0i8.byte_size(), 1);
    }

    #[test]
    fn test_byte_size_i16() {
        assert_eq!(0i16.byte_size(), 2);
    }

    #[test]
    fn test_byte_size_i32() {
        assert_eq!(0i32.byte_size(), 4);
    }

    #[test]
    fn test_byte_size_i64() {
        assert_eq!(0i64.byte_size(), 8);
    }

    #[test]
    fn test_byte_size_i128() {
        assert_eq!(0i128.byte_size(), 16);
    }

    #[test]
    fn test_byte_size_f32() {
        assert_eq!(0.0f32.byte_size(), 4);
    }

    #[test]
    fn test_byte_size_f64() {
        assert_eq!(0.0f64.byte_size(), 8);
    }

    // Unit
    #[test]
    fn test_byte_size_unit() {
        assert_eq!(().byte_size(), 0);
    }

    // Arrays
    #[test]
    fn test_byte_size_array_u8() {
        let arr: [u8; 4] = [1, 2, 3, 4];
        assert_eq!(arr.byte_size(), 4);
    }

    #[test]
    fn test_byte_size_array_u32() {
        let arr: [u32; 3] = [1, 2, 3];
        assert_eq!(arr.byte_size(), 12);
    }

    #[test]
    fn test_byte_size_array_empty() {
        let arr: [u8; 0] = [];
        assert_eq!(arr.byte_size(), 0);
    }

    // str
    #[test]
    fn test_byte_size_str_empty() {
        assert_eq!("".byte_size(), 1); // just the 0x00 prefix
    }

    #[test]
    fn test_byte_size_str_short() {
        // 1 (0x0b prefix) + 1 (uleb128 len for 5) + 5 (bytes)
        assert_eq!("hello".byte_size(), 7);
    }

    #[test]
    fn test_byte_size_str_single_char() {
        // 1 (0x0b) + 1 (uleb128 len for 1) + 1 (byte)
        assert_eq!("a".byte_size(), 3);
    }

    // String
    #[test]
    fn test_byte_size_string_empty() {
        let s = String::new();
        assert_eq!(s.byte_size(), 1);
    }

    #[test]
    fn test_byte_size_string_nonempty() {
        let s = String::from("test");
        // 1 (0x0b) + 1 (uleb128 len for 4) + 4 (bytes)
        assert_eq!(s.byte_size(), 6);
    }
}
