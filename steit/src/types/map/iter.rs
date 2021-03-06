use std::marker::PhantomData;

use super::key::MapKey;

pub struct MapIter<'a, K: MapKey, V: 'a> {
    inner: Box<dyn Iterator<Item = (&'a u32, &'a V)> + 'a>,
    _marker: PhantomData<*const K>,
}

impl<'a, K: MapKey, V> MapIter<'a, K, V> {
    pub(super) fn new(inner: impl Iterator<Item = (&'a u32, &'a V)> + 'a) -> Self {
        Self {
            inner: Box::new(inner),
            _marker: PhantomData,
        }
    }
}

impl<'a, K: MapKey, V> Iterator for MapIter<'a, K, V> {
    type Item = (K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(&field_number, value)| {
            let key = K::try_from_field_number(field_number).unwrap();
            (key, value)
        })
    }
}

pub struct MapIterMut<'a, K: MapKey, V: 'a> {
    inner: Box<dyn Iterator<Item = (&'a u32, &'a mut V)> + 'a>,
    _marker: PhantomData<*const K>,
}

impl<'a, K: MapKey, V> MapIterMut<'a, K, V> {
    pub(super) fn new(inner: impl Iterator<Item = (&'a u32, &'a mut V)> + 'a) -> Self {
        Self {
            inner: Box::new(inner),
            _marker: PhantomData,
        }
    }
}

impl<'a, K: MapKey, V> Iterator for MapIterMut<'a, K, V> {
    type Item = (K, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(&field_number, value)| {
            let key = K::try_from_field_number(field_number).unwrap();
            (key, value)
        })
    }
}
