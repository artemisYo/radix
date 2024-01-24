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
    pub fn next_idx(&self) -> K {
        K::from(self.0.len()).unwrap()
    }
    pub fn current_idx(&self) -> K {
        K::from(self.0.len() - 1).unwrap()
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

pub struct JaggedVec<K, T>(Vec<T>, PhantomData<K>);
impl<K, T> JaggedVec<K, T> {
    pub fn new() -> Self {
        Self(Vec::new(), PhantomData)
    }
}
impl<K: KeyChain, T> JaggedVec<K, T> {
    pub fn push_slice(&mut self, slice: &[T]) -> K
    where
        T: Clone,
    {
        let idx = self.0.len();
        self.0.extend_from_slice(slice);
        let len = self.0.len();
        K::from(idx, len).unwrap()
    }
    pub fn push_iter<I>(&mut self, iter: I) -> K
    where
        I: IntoIterator<Item = T>,
    {
        let idx = self.0.len();
        self.0.extend(iter);
        let len = self.0.len();
        K::from(idx, len).unwrap()
    }
    pub fn push_ref_iter<'a, I>(&mut self, iter: I) -> K
    where
        T: Copy + 'a,
        I: IntoIterator<Item = &'a T>,
    {
        let idx = self.0.len();
        self.0.extend(iter);
        let len = self.0.len();
        K::from(idx, len).unwrap()
    }
}
impl<K, T> std::iter::Extend<T> for JaggedVec<K, T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.0.extend(iter)
    }
}
impl<'a, K, T: Copy + 'a> std::iter::Extend<&'a T> for JaggedVec<K, T> {
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        self.0.extend(iter)
    }
}
impl<K: KeyChain, T> std::ops::Index<K> for JaggedVec<K, T> {
    type Output = [T];

    fn index(&self, index: K) -> &Self::Output {
        let (start, end) = index.into();
        &self.0[start..end]
    }
}
impl<K: KeyChain, T> std::ops::IndexMut<K> for JaggedVec<K, T> {
    fn index_mut(&mut self, index: K) -> &mut Self::Output {
        let (start, end) = index.into();
        &mut self.0[start..end]
    }
}
