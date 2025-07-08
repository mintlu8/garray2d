use mint::Vector2;

use crate::Zip;
use crate::traits::{Array2dStorageMut, Array2dStorageOwned};

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
        self.iter::<U>()
            .filter_map(|(pos, is_true)| is_true.then_some(pos))
    }

    /// For a boolean 2d array, iterate through points with `true` values.
    pub fn iter_points_owned<U: From<Vector2<i32>>>(self) -> impl Iterator<Item = U>
    where
        T: Array2dStorageOwned,
    {
        self.iter_owned::<U>()
            .filter_map(|(pos, is_true)| is_true.then_some(pos))
    }
}

impl<T: Array2dStorage<Item = Option<A>>, A> GenericArray2d<T> {
    /// For a option 2d array, iterate through points with `Some` values.
    pub fn iter_some<'t, U: From<Vector2<i32>>>(&'t self) -> impl Iterator<Item = (U, &'t A)>
    where
        A: 't,
    {
        self.iter::<U>()
            .filter_map(|(pos, value)| value.as_ref().map(|v| (pos, v)))
    }

    /// For a option 2d array, iterate through points with `Some` values.
    pub fn iter_some_mut<'t, U: From<Vector2<i32>>>(
        &'t mut self,
    ) -> impl Iterator<Item = (U, &'t mut A)>
    where
        T: Array2dStorageMut,
        A: 't,
    {
        self.iter_mut::<U>()
            .filter_map(|(pos, value)| value.as_mut().map(|v| (pos, v)))
    }

    /// For a boolean 2d array, iterate through points with `Some` values.
    pub fn iter_some_owned<U: From<Vector2<i32>>>(self) -> impl Iterator<Item = (U, A)>
    where
        T: Array2dStorageOwned,
    {
        self.iter_owned::<U>()
            .filter_map(|(pos, value)| value.map(|v| (pos, v)))
    }
}
