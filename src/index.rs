use mint::Vector2;

use crate::util::*;
use crate::{
    Array2dMut, Array2dRef, Boundary, GenericArray2d,
    boundary::IntoBoundary,
    storage::{Array2dStorage, Array2dStorageMut},
};

/// Make an item usable in `get` and `get_mut`.
///
/// Implemented on `impl Into<mint::Vector<i32>>`,
/// or `impl IntoBoundary`
/// using type inference, see [`IntoBoundary`].
///
/// Any type that implements more than one of these will cause `get` to fail.
pub trait Array2dIndexing<Marker> {
    type Result<'t, T>
    where
        T: 't;
    type ResultMut<'t, T>
    where
        T: 't;
    fn index<'t, T: Array2dStorage>(
        self,
        array: &'t GenericArray2d<T>,
    ) -> Self::Result<'t, T::Item>;
    fn index_mut<'t, T: Array2dStorageMut>(
        self,
        array: &'t mut GenericArray2d<T>,
    ) -> Self::ResultMut<'t, T::Item>;
}

pub struct Vector2Marker;
pub struct BoundaryMarker;

impl<U: Into<Vector2<i32>>> Array2dIndexing<Vector2Marker> for U {
    type Result<'t, T>
        = Option<&'t T>
    where
        T: 't;

    type ResultMut<'t, T>
        = Option<&'t mut T>
    where
        T: 't;

    fn index<'t, T: Array2dStorage>(
        self,
        array: &'t GenericArray2d<T>,
    ) -> Self::Result<'t, T::Item> {
        let v: Vector2<i32> = self.into();
        let x = v.x - array.boundary.min.x;
        let y = v.y - array.boundary.min.y;
        if x < 0 || x >= array.boundary.dimension.x as i32 {
            return None;
        }
        if y < 0 || y >= array.boundary.dimension.y as i32 {
            return None;
        }
        array
            .data
            .slice()
            .get(y as usize * array.pitch + x as usize)
    }

    fn index_mut<'t, T: Array2dStorageMut>(
        self,
        array: &'t mut GenericArray2d<T>,
    ) -> Self::ResultMut<'t, T::Item> {
        let v: Vector2<i32> = self.into();
        let x = v.x - array.boundary.min.x;
        let y = v.y - array.boundary.min.y;
        if x < 0 || x >= array.boundary.dimension.x as i32 {
            return None;
        }
        if y < 0 || y >= array.boundary.dimension.y as i32 {
            return None;
        }
        array
            .data
            .slice_mut()
            .get_mut(y as usize * array.pitch + x as usize)
    }
}

impl<U: IntoBoundary> Array2dIndexing<BoundaryMarker> for U {
    type Result<'t, T>
        = Option<Array2dRef<'t, T>>
    where
        T: 't;
    type ResultMut<'t, T>
        = Option<Array2dMut<'t, T>>
    where
        T: 't;

    fn index<'t, T: Array2dStorage>(
        self,
        array: &'t GenericArray2d<T>,
    ) -> Self::Result<'t, T::Item> {
        let (ok, slice) = array.slice_internal(self.into_boundary());
        ok.then_some(slice)
    }

    fn index_mut<'t, T: Array2dStorageMut>(
        self,
        array: &'t mut GenericArray2d<T>,
    ) -> Self::ResultMut<'t, T::Item> {
        let (ok, slice) = array.slice_mut_internal(self.into_boundary());
        ok.then_some(slice)
    }
}

impl<T: Array2dStorage> GenericArray2d<T> {
    pub(crate) fn slice_internal(&self, input: Boundary) -> (bool, Array2dRef<T::Item>) {
        if let Some(intersection) = self.boundary.intersection(input) {
            let min = sub(intersection.min, self.boundary.min);
            let offset = (min.y * self.pitch as i32 + min.x) as usize;
            let is_perfect = intersection == input;
            (
                is_perfect,
                Array2dRef {
                    data: &self.data.slice()[offset..],
                    boundary: intersection,
                    pitch: self.pitch,
                },
            )
        } else {
            (false, Array2dRef::default())
        }
    }
}

impl<T: Array2dStorageMut> GenericArray2d<T> {
    pub(crate) fn slice_mut_internal(&mut self, input: Boundary) -> (bool, Array2dMut<T::Item>) {
        if let Some(intersection) = self.boundary.intersection(input) {
            let min = sub(intersection.min, self.boundary.min);
            let offset = (min.y * self.pitch as i32 + min.x) as usize;
            let is_perfect = intersection == input;
            (
                is_perfect,
                Array2dMut {
                    data: &mut self.data.slice_mut()[offset..],
                    boundary: intersection,
                    pitch: self.pitch,
                },
            )
        } else {
            (false, Array2dMut::default())
        }
    }
}
