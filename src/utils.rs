use std::marker::PhantomData;

#[macro_export] macro_rules! MakeKey {
    ($name:ident, $base_type:path) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $name($base_type);
        impl From<usize> for $name {
            fn from(value: usize) -> Self {
                Self(value.try_into().expect("Could not convert into key type!"))
            }
        }
        impl crate::utils::Key for $name {
            fn index<T>(self, s: &[T]) -> &T {
                &s[TryInto::<usize>::try_into(self.0)
                    .expect("Could not convert out of key type!")]
            }
            fn index_mut<T>(self, s: &mut [T]) -> &mut T {
                &mut s[TryInto::<usize>::try_into(self.0)
                    .expect("Could not convert out of key type!")]
            }
        }
    };
}
#[macro_export] macro_rules! MakeRange {
    ($name:ident, $base_type:path, $len_type:path) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $name($base_type, $len_type);
        impl From<(usize, usize)> for $name {
            fn from(value: (usize, usize)) -> Self {
                let zero = value.0.try_into().expect("Could not convert into key type!");
                let one = value.1.try_into().expect("Could not convert into key type!");
                Self(zero, one)
            }
        }
        impl crate::utils::Range for $name {
            fn index<T>(self, s: &[T]) -> &[T] {
                let start: usize = self.0.try_into().expect("Could not convert out of key type!");
                let len: usize = self.1.try_into().expect("Could not convert out of key type!");
                &s[start..start + len]
            }
        }
    };
}

pub trait Key: From<usize> {
    fn index<'a, T>(self, _: &'a [T]) -> &'a T;
    fn index_mut<T>(self, s: &mut [T]) -> &mut T;
}
pub trait Range: From<(usize, usize)> {
    fn index<'a, T>(self, _: &'a [T]) -> &'a [T];
}

pub trait Plurality {}
#[derive(Debug)]
pub struct Single;
impl Plurality for Single {}
#[derive(Debug)]
pub struct Multiple;
impl Plurality for Multiple {}

#[derive(Debug)]
pub struct KeyVec<F: Plurality, K, V>(PhantomData<(F, K)>, Vec<V>);
impl<K: Key, V> KeyVec<Single, K, V> {
    pub fn new() -> Self {
        Self(PhantomData, Vec::new())
    }
    pub fn iter(&self) -> std::slice::Iter<'_, V> {
        self.1.iter()
    }
    pub fn len(&self) -> usize {
        self.1.len()
    }
    pub fn last(&self) -> Option<&V> {
        self.1.last()
    }
    pub fn last_mut(&mut self) -> Option<&mut V> {
        self.1.last_mut()
    }
}
impl<K: Range, V> KeyVec<Multiple, K, V> {
    pub fn new() -> Self {
        Self(PhantomData, Vec::new())
    }
}
impl<K: Key, V> KeyVec<Single, K, V> {
    pub fn push(&mut self, value: V) -> K {
        let out = self.1.len().into();
        self.1.push(value);
        out
    }
    pub fn last_key(&self) -> K {
        (self.1.len() - 1).into()
    }
    pub fn new_key(&self) -> K {
		self.1.len().into()
    }
}
impl<K: Range, V: Clone> KeyVec<Multiple, K, V> {
    pub fn append(&mut self, value: &[V]) -> K {
        let out = (self.1.len(), value.len()).into();
        self.1.extend_from_slice(value);
        out
    }
    pub fn append_from_iter(&mut self, value: impl Iterator<Item = V>) -> K {
        let start = self.1.len();
        self.1.extend(value);
        let end = self.1.len() - start;
        (start, end).into()
    }
}

impl<K: Key, V> std::ops::Index<K> for KeyVec<Single, K, V> {
    type Output = V;
    fn index(&self, index: K) -> &Self::Output {
        index.index(self.1.as_slice())
    }
}
impl<K: Key, V> std::ops::IndexMut<K> for KeyVec<Single, K, V> {
    fn index_mut(&mut self, index: K) -> &mut Self::Output {
        index.index_mut(self.1.as_mut_slice())
    }
}
impl<K: Range, V> std::ops::Index<K> for KeyVec<Multiple, K, V> {
    type Output = [V];
    fn index(&self, index: K) -> &Self::Output {
        index.index(self.1.as_slice())
    }
}

impl<K: Key, V> IntoIterator for KeyVec<Single, K, V> {
    type Item = V;

    type IntoIter = <Vec<V> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.1.into_iter()
    }
}

