use mint::Vector2;

use crate::Zip;
use crate::traits::Array2dStorageMut;

use crate::{Array2d, GenericArray2d, traits::Array2dStorage, zip::GenericArray2dRef};

impl<T: Array2dStorage> GenericArray2d<T> {
    /// Copy the array or slice into an owned array.
    pub fn copied(&self) -> Array2d<T::Item>
    where
        T::Item: Copy,
    {
        let mut data = Vec::with_capacity(self.len());
        data.extend(self.values().copied());
        Array2d {
            data,
            boundary: self.boundary,
            pitch: self.pitch,
        }
    }

    /// Clone the array or slice into an owned array.
    pub fn cloned(&self) -> Array2d<T::Item>
    where
        T::Item: Clone,
    {
        let mut data = Vec::with_capacity(self.len());
        data.extend(self.values().cloned());
        Array2d {
            data,
            boundary: self.boundary,
            pitch: self.pitch,
        }
    }

    /// Map the array or slice into an owned array.
    pub fn mapped<U>(&self, f: impl FnMut(&T::Item) -> U) -> Array2d<U> {
        let mut data = Vec::with_capacity(self.len());
        data.extend(self.values().map(f));
        Array2d {
            data,
            boundary: self.boundary,
            pitch: self.pitch,
        }
    }

    /// Combine with another array, must have the same dimension.
    ///
    /// Supports both mutable and immutable references.
    pub fn zip<U: GenericArray2dRef>(&self, rhs: U) -> Zip<&Self, U> {
        Zip(self, rhs)
    }

    /// Combine with another array, must have the same dimension.
    ///
    /// Supports both mutable and immutable references.
    pub fn zip_mut<U: GenericArray2dRef>(&mut self, rhs: U) -> Zip<&mut Self, U>
    where
        T: Array2dStorageMut,
    {
        Zip(self, rhs)
    }
}

impl<T: Array2dStorage<Item = bool>> GenericArray2d<T> {
    /// For a boolean 2d array, iterate through points with `true` values.
    pub fn iter_points<U: From<Vector2<i32>>>(&self) -> impl Iterator<Item = U> {
        self.iter::<U>().filter_map(|(pos, is_true)| is_true.then_some(pos))
    }
}
