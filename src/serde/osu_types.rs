use core::marker::PhantomData;
use std::fmt::Debug;
use crate::serde::byte_sized::ByteSized;
use crate::serde::deserialize::{BinaryDeserialize, BinaryReader};
use crate::serde::serialize::{BinarySerialize, BinaryWriter};


#[derive(Debug)]
pub struct PrefixedVec<P: TryFrom<usize> + Default + Debug, T>(
    Vec<T>,
    PhantomData<P>,
);

// Serialize
impl<P: TryFrom<usize> + Default + Debug, T> PrefixedVec<P, T> {
    pub fn new() -> Self {
        Self(vec![], PhantomData)
    }
}

impl<P: TryFrom<usize> + Default + Debug, T> From<Vec<T>> for PrefixedVec<P, T> {
    fn from(value: Vec<T>) -> Self {
        Self(value, PhantomData)
    }
}

impl<P: BinarySerialize + TryFrom<usize> + Default + Debug, T: BinarySerialize> ByteSized for PrefixedVec<P, T> {
    fn byte_size(&self) -> usize {
        P::default().byte_size() + self.0.iter().map(|v| v.byte_size()).sum::<usize>()
    }
}

impl<P: BinarySerialize + TryFrom<usize> + Default + Debug, T: BinarySerialize> BinarySerialize for PrefixedVec<P, T> {
    fn write_to(&self, writer: &mut BinaryWriter) {
        let len = P::try_from(self.0.len()).unwrap_or_default();
        P::write_to(&len, writer);

        for v in &self.0 {
            T::write_to(v, writer);
        }
    }
}


// Deserialize
impl<'a, P: BinaryDeserialize<'a> + TryInto<usize> + TryFrom<usize> + Default + Debug, T: BinaryDeserialize<'a>> BinaryDeserialize<'a> for PrefixedVec<P, T>
    where
        <P as TryInto<usize>>::Error: Debug
{
    fn read_from(reader: &mut BinaryReader<'a>) -> std::io::Result<Self>
        where
            Self: Sized
    {
        let len = P::read_from(reader)?;
        let len = TryInto::<usize>::try_into(len).unwrap();
        let mut r = vec![];
        for _i in 0..len {
            r.push(T::read_from(reader)?);
        }
        Ok(Self::from(r))
    }
}