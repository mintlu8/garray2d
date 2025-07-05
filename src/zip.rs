//! Operations on arrays with the same dimension.

use crate::{
    storage::{Array2dStorage, Array2dStorageMut}, util::*, Array2d, Boundary, GenericArray2d
};
use mint::Vector2;

/// Unifies `&GenericArray2d` and `&mut GenericArray2d` into the same interface for [`Zip`].
pub trait GenericArray2dRef {
    type Row<'t>: IntoIterator
    where
        Self: 't;
    type RowMut<'t>: IntoIterator
    where
        Self: 't;
    fn min(&self) -> Vector2<i32>;
    fn dimension(&self) -> Vector2<u32>;
    fn rows(&self) -> impl Iterator<Item = Self::Row<'_>>;
    fn rows_mut(&mut self) -> impl Iterator<Item = Self::RowMut<'_>>;
}

impl<T: Array2dStorage> GenericArray2dRef for &GenericArray2d<T> {
    type Row<'t>
        = &'t [T::Item]
    where
        Self: 't;
    type RowMut<'t>
        = &'t [T::Item]
    where
        Self: 't;
    fn min(&self) -> Vector2<i32> {
        self.boundary.min
    }

    fn dimension(&self) -> Vector2<u32> {
        self.boundary.dimension
    }

    fn rows(&self) -> impl Iterator<Item = Self::Row<'_>> {
        GenericArray2d::rows(self)
    }

    fn rows_mut(&mut self) -> impl Iterator<Item = Self::RowMut<'_>> {
        GenericArray2d::rows(self)
    }
}

impl<T: Array2dStorageMut> GenericArray2dRef for &mut GenericArray2d<T> {
    type Row<'t>
        = &'t [T::Item]
    where
        Self: 't;
    type RowMut<'t>
        = &'t mut [T::Item]
    where
        Self: 't;
    fn min(&self) -> Vector2<i32> {
        self.boundary.min
    }

    fn dimension(&self) -> Vector2<u32> {
        self.boundary.dimension
    }

    fn rows(&self) -> impl Iterator<Item = Self::Row<'_>> {
        GenericArray2d::rows(self)
    }

    fn rows_mut(&mut self) -> impl Iterator<Item = Self::RowMut<'_>> {
        GenericArray2d::rows_mut(self)
    }
}

impl<A: Array2dStorage<Item: PartialEq<B::Item>>, B:Array2dStorage> PartialEq<GenericArray2d<B>> for GenericArray2d<A> {
    fn eq(&self, other: &GenericArray2d<B>) -> bool {
        self.boundary == other.boundary && self.rows().zip(other.rows()).all(|(a, b)| a == b)
    }
}

impl<T: Array2dStorage<Item: Eq>> Eq for GenericArray2d<T> {}

impl<T: Array2dStorage<Item: Eq>> GenericArray2d<T> {
    /// Returns true if dimension and underlying data are equal, ignores the origin points.
    pub fn equivalent<U: Array2dStorage>(&self, other: &GenericArray2d<U>) -> bool where T::Item: PartialEq<U::Item> {
        self.boundary.dimension == other.boundary.dimension 
            && self.rows().zip(other.rows()).all(|(a, b)| a == b)
    }
}

/// Zipped references of 2d arrays of the same dimension.
///
/// Supports `&array` or `&mut array` only if underlying data is mutable.
/// 
/// # Constraints 
/// 
/// Dimensions of both arrays must match, origin points are not considered.
pub struct Zip<A: GenericArray2dRef, B: GenericArray2dRef>(pub A, pub B);

type Item<'t, T> = <<T as GenericArray2dRef>::Row<'t> as IntoIterator>::Item;
type ItemMut<'t, T> = <<T as GenericArray2dRef>::RowMut<'t> as IntoIterator>::Item;

impl<A: GenericArray2dRef, B: GenericArray2dRef> Zip<A, B> {
    /// Returns true if size matches.
    /// 
    /// This is required for all operations on this type.
    pub fn is_valid(&self) -> bool {
        self.0.dimension() == self.1.dimension()
    }

    /// Returns false and has no effect if the arrays do not have equal dimension.
    pub fn for_each(&self, mut f: impl FnMut(Item<'_, A>, Item<'_, B>)) -> bool {
        if self.0.dimension() != self.1.dimension() {
            return false;
        }
        for (row_0, row_1) in self.0.rows().zip(self.1.rows()) {
            for (a, b) in row_0.into_iter().zip(row_1) {
                f(a, b)
            }
        }
        true
    }

    /// Returns false and has no effect if the arrays do not have equal dimension.
    pub fn for_each_mut(&mut self, mut f: impl FnMut(ItemMut<'_, A>, ItemMut<'_, B>)) -> bool {
        if self.0.dimension() != self.1.dimension() {
            return false;
        }
        for (row_0, row_1) in self.0.rows_mut().zip(self.1.rows_mut()) {
            for (a, b) in row_0.into_iter().zip(row_1) {
                f(a, b)
            }
        }
        true
    }

    /// Returns false and has no effect if the arrays do not have equal dimension.
    pub fn for_each_indexed<I: From<Vector2<i32>>>(
        &self,
        mut f: impl FnMut(I, Item<'_, A>, I, Item<'_, B>),
    ) -> bool {
        if self.0.dimension() != self.1.dimension() {
            return false;
        }
        let min0 = self.0.min();
        let min1 = self.1.min();
        for (x, (row_0, row_1)) in self.0.rows().zip(self.1.rows()).enumerate() {
            for (y, (a, b)) in row_0.into_iter().zip(row_1).enumerate() {
                let pos = Vector2 {
                    x: x as i32,
                    y: y as i32,
                };
                f(add(pos, min0).into(), a, add(pos, min1).into(), b);
            }
        }
        true
    }

    /// Returns false and has no effect if the arrays do not have equal dimension.
    pub fn for_each_indexed_mut<I: From<Vector2<i32>>>(
        &mut self,
        mut f: impl FnMut(I, ItemMut<'_, A>, I, ItemMut<'_, B>),
    ) -> bool {
        if self.0.dimension() != self.1.dimension() {
            return false;
        }
        let min0 = self.0.min();
        let min1 = self.1.min();
        for (x, (row_0, row_1)) in self.0.rows_mut().zip(self.1.rows_mut()).enumerate() {
            for (y, (a, b)) in row_0.into_iter().zip(row_1).enumerate() {
                let pos = Vector2 {
                    x: x as i32,
                    y: y as i32,
                };
                f(add(pos, min0).into(), a, add(pos, min1).into(), b);
            }
        }
        true
    }

    /// Create a new array by combining the two, inheriting the position of the first array.
    /// 
    /// # Panics
    /// 
    /// If dimension mismatch.
    #[track_caller]
    pub fn map<U>(&self, mut f: impl FnMut(Item<'_, A>, Item<'_, B>) -> U) -> Array2d<U> {
        if self.0.dimension() != self.1.dimension() {
            panic!("Dimension mismatch!");
        }
        let dimension = self.0.dimension();
        let mut result = Vec::with_capacity((dimension.x * dimension.y) as usize);
        for (row_0, row_1) in self.0.rows().zip(self.1.rows()) {
            for (a, b) in row_0.into_iter().zip(row_1) {
                result.push(f(a, b))
            }
        }
        let boundary = Boundary {
            min: self.0.min(),
            dimension,
        };
        Array2d {
            data: result,
            boundary,
            pitch: boundary.pitch(),
        }
    }

    /// Create a new array by combining the two, inheriting the position of the first array.
    /// 
    /// # Panics
    /// 
    /// If dimension mismatch.
    #[track_caller]
    pub fn map_mut<U>(&mut self, mut f: impl FnMut(ItemMut<'_, A>, ItemMut<'_, B>) -> U) -> Array2d<U> {
        if self.0.dimension() != self.1.dimension() {
            panic!("Dimension mismatch!");
        }
        let dimension = self.0.dimension();
        let mut result = Vec::with_capacity((dimension.x * dimension.y) as usize);
        for (row_0, row_1) in self.0.rows_mut().zip(self.1.rows_mut()) {
            for (a, b) in row_0.into_iter().zip(row_1) {
                result.push(f(a, b))
            }
        }
        let boundary = Boundary {
            min: self.0.min(),
            dimension,
        };
        Array2d {
            data: result,
            boundary,
            pitch: boundary.pitch(),
        }
    }
}
