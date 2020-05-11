use std::io;

use crate::{
    de_v2::{DeserializeV2, Reader},
    rt::SizeCache,
    ser_v2::SerializeV2,
    wire_fmt::{HasWireType, WireTypeV2},
};

impl<T> HasWireType for Option<T> {
    const WIRE_TYPE: WireTypeV2 = WireTypeV2::Sized;
}

impl<T: SerializeV2> SerializeV2 for Option<T> {
    fn compute_size_v2(&self) -> u32 {
        match self {
            Some(value) => value.compute_size_nested_v2(None, false).unwrap(),
            None => 0,
        }
    }

    fn serialize_cached(&self, writer: &mut impl io::Write) -> io::Result<()> {
        match self {
            Some(value) => value.serialize_nested(None, false, writer),
            None => Ok(()),
        }
    }

    #[inline]
    fn size_cache(&self) -> Option<&SizeCache> {
        None
    }
}

impl<T: DeserializeV2> DeserializeV2 for Option<T> {
    #[inline]
    fn merge_v2(&mut self, reader: &mut Reader<impl io::Read>) -> io::Result<()> {
        while !reader.eof()? {
            if self.is_none() {
                *self = Some(T::default());
            }

            if let Some(value) = self {
                value.merge_nested_v2(T::WIRE_TYPE, reader)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ser_v2::SerializeV2,
        test_case,
        test_util_v2::{assert_merge, assert_serialize, assert_serialize_nested, assert_size, Foo},
    };

    #[test]
    fn cached_size() {
        let value = Some(Foo::new(-1, 0));
        assert_eq!(value.as_ref().unwrap().cached_size(), 0);
        assert_eq!(value.cache_size(), 3);
        assert_eq!(value.unwrap().cached_size(), 2);
    }

    test_case!(size_01: assert_size; None::<u8> => 0);
    test_case!(size_02: assert_size; Some(0) => 1);
    test_case!(size_03: assert_size; Some(1) => 1);
    test_case!(size_04: assert_size; Some(1337) => 2);

    test_case!(serialize_01: assert_serialize; None::<u8> => &[]);
    test_case!(serialize_02: assert_serialize; Some(0) => &[0]);
    test_case!(serialize_03: assert_serialize; Some(1337) => &[242, 20]);
    test_case!(serialize_04: assert_serialize; Some(Foo::empty()) => &[0]);
    test_case!(serialize_05: assert_serialize; Some(Foo::new(-1, -2)) => &[4, 0, 1, 8, 3]);

    test_case!(serialize_nested_01: assert_serialize_nested; None::<u8>, None => &[0]);
    test_case!(serialize_nested_02: assert_serialize_nested; Some(1), None => &[1, 2]);
    test_case!(serialize_nested_03: assert_serialize_nested; None::<u8>, Some(10) => &[]);
    test_case!(serialize_nested_04: assert_serialize_nested; Some(1), Some(10) => &[82, 1, 2]);
    test_case!(serialize_nested_05: assert_serialize_nested; Some(Foo::empty()), Some(10) => &[82, 1, 0]);
    test_case!(serialize_nested_06: assert_serialize_nested; Some(Foo::new(-1, -2)), Some(10) => &[82, 5, 4, 0, 1, 8, 3]);

    test_case!(merge_01: assert_merge; Some(1), &[] => Some(1));
    test_case!(merge_02: assert_merge; None, &[0] => Some(0));
    test_case!(merge_03: assert_merge; Some(0), &[242, 20] => Some(1337));
    test_case!(merge_04: assert_merge; Some(Foo::new(-1, -2)), &[2, 8, 4] => Some(Foo::new(-1, 2)));
    test_case!(merge_05: assert_merge; None, &[2, 8, 4] => Some(Foo::new(0, 2)));
}
