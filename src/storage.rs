/// A readable slice storage of `GenericArray2d`.
pub trait Array2dStorage {
    type Item;
    fn slice(&self) -> &[Self::Item];
}

/// A mutable slice storage of `GenericArray2d`.
pub trait Array2dStorageMut: Array2dStorage {
    fn slice_mut(&mut self) -> &mut [Self::Item];
}

/// An owned growable storage of `GenericArray2d`.
pub trait Array2dStorageOwned: Array2dStorageMut {
    fn vec_mut(&mut self) -> &mut Vec<Self::Item>;
    fn from_vec(vec: Vec<Self::Item>) -> Self;
}

impl<T> Array2dStorage for &[T] {
    type Item = T;

    fn slice(&self) -> &[Self::Item] {
        self
    }
}

impl<T> Array2dStorage for &mut [T] {
    type Item = T;

    fn slice(&self) -> &[Self::Item] {
        self
    }
}

impl<T> Array2dStorageMut for &mut [T] {
    fn slice_mut(&mut self) -> &mut [Self::Item] {
        self
    }
}

impl<T> Array2dStorage for Vec<T> {
    type Item = T;

    fn slice(&self) -> &[Self::Item] {
        self
    }
}

impl<T> Array2dStorageMut for Vec<T> {
    fn slice_mut(&mut self) -> &mut [Self::Item] {
        self
    }
}

impl<T> Array2dStorageOwned for Vec<T> {
    fn vec_mut(&mut self) -> &mut Vec<Self::Item> {
        self
    }

    fn from_vec(vec: Vec<Self::Item>) -> Self {
        vec
    }
}
