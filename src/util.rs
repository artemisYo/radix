use std::marker::PhantomData;

// these are used for some type-level guards
// i.e. `Block` handles allow inserting instructions only once
//      by changing the parameter from False to True
//      or allow sealing the block the same way
pub trait TypeBool {}
pub struct True;
impl TypeBool for True {}
pub struct False;
impl TypeBool for False {}

pub trait Key {
    fn from(index: usize) -> Option<Self>
    where
        Self: Sized;
    fn into(self) -> usize;
}
pub trait KeyChain {
    fn from(idx: usize, len: usize) -> Option<Self>
    where
        Self: Sized;
    fn into(self) -> (usize, usize);
}
pub struct KeyVec<K, T>(Vec<T>, PhantomData<K>);
impl<K, T> KeyVec<K, T> {
    pub fn new() -> Self {
        Self(Vec::new(), PhantomData)
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.0.iter()
    }
}
impl<K: Key, T> KeyVec<K, T> {
    pub fn push(&mut self, elem: T) -> K {
        // panic here as if we are out of addressing range
        // then we're just sort of fucked
        let k = self.next_idx();
        self.0.push(elem);
        k
    }
    pub fn push_slice(&mut self, elem: &[T]) -> [K; 2]
    where T: Clone {
        let start = self.next_idx();
        self.0.extend_from_slice(elem);
        let end = self.next_idx();
        [start, end]
    }
    pub fn next_idx(&self) -> K {
        K::from(self.0.len()).unwrap()
    }
    pub fn get(&self, index: K) -> Option<&T> {
        self.0.get(index.into())
    }
}
impl<K: Key, T> std::ops::Index<K> for KeyVec<K, T> {
    type Output = T;

    fn index(&self, index: K) -> &Self::Output {
        &self.0[index.into()]
    }
}
impl<K: Key, T> std::ops::IndexMut<K> for KeyVec<K, T> {
    fn index_mut(&mut self, index: K) -> &mut Self::Output {
        &mut self.0[index.into()]
    }
}
impl<K: Key, T> std::ops::Index<(K, K)> for KeyVec<K, T> {
    type Output = [T];

    fn index(&self, index: (K, K)) -> &Self::Output {
        let (start, end) = index;
        &self.0[start.into()..end.into()]
    }
}
impl<K: Key, T> std::ops::IndexMut<(K, K)> for KeyVec<K, T> {
    fn index_mut(&mut self, index: (K, K)) -> &mut Self::Output {
        let (start, end) = index;
        &mut self.0[start.into()..end.into()]
    }
}
impl<K: Key, T> std::ops::Index<[K; 2]> for KeyVec<K, T> {
    type Output = [T];

    fn index(&self, index: [K; 2]) -> &Self::Output {
        let [start, end] = index;
        &self.0[start.into()..end.into()]
    }
}
impl<K: Key, T> std::ops::IndexMut<[K; 2]> for KeyVec<K, T> {
    fn index_mut(&mut self, index: [K; 2]) -> &mut Self::Output {
        let [start, end] = index;
        &mut self.0[start.into()..end.into()]
    }
}
