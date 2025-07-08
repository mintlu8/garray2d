use crate::index::Array2dIndexing;
use crate::storage::{Array2dStorage, Array2dStorageMut, Array2dStorageOwned};
use crate::util::*;
use crate::{Array2dMut, Array2dRef, Boundary, GenericArray2d, IntoBoundary};
use mint::Vector2;
impl<T: Array2dStorage> GenericArray2d<T> {
    /// Returns true if contains no items.
    pub fn is_empty(&self) -> bool {
        self.boundary.is_empty()
    }

    /// Returns the number of items in the array.
    pub fn len(&self) -> usize {
        self.boundary.len()
    }

    /// Returns the width of the array.
    pub fn width(&self) -> usize {
        self.boundary.dimension.x as usize
    }

    /// Returns the height of the array.
    pub fn height(&self) -> usize {
        self.boundary.dimension.y as usize
    }

    /// Returns the numerically smallest coordinate in the array.
    pub fn min_point<U: From<Vector2<i32>>>(&self) -> U {
        self.boundary.min.into()
    }

    /// Returns the numerically largest coordinate in the array.
    ///
    /// # Note
    ///
    /// Equivalent to `min_point + dimension - [1, 1]`.
    pub fn max_point<U: From<Vector2<i32>>>(&self) -> U {
        self.boundary.max().into()
    }

    /// Returns the dimension of the array.
    ///
    /// # Note
    ///
    /// Equivalent to `max_point - min_point + [1, 1]`.
    pub fn dimension<U: From<Vector2<u32>>>(&self) -> U {
        self.boundary.dimension.into()
    }

    /// Returns the boundary of the array.
    pub fn boundary(&self) -> Boundary {
        self.boundary
    }

    /// Returns true if a point is in boundary.
    pub fn contains(&self, position: impl Into<Vector2<i32>>) -> bool {
        self.boundary.contains(position)
    }

    /// Returns either a point or a slice via [`IntoBoundary`].
    ///
    /// Unlike [`slice`](GenericArray2d::slice), this only returns `Some` if all points are contained in the array.
    pub fn get<I: Array2dIndexing<M>, M>(&self, point: I) -> I::Result<'_, T::Item> {
        point.index(self)
    }

    /// Shorthand to `self.get(point).cloned().unwrap_or_default()`.
    pub fn fetch(&self, point: impl Into<Vector2<i32>>) -> T::Item
    where
        T::Item: Clone + Default,
    {
        point.index(self).cloned().unwrap_or_default()
    }

    /// Iterate through pairs of points and values in the array.
    pub fn iter<U: From<Vector2<i32>>>(&self) -> impl Iterator<Item = (U, &T::Item)> {
        let min = self.boundary.min;
        DimensionIter::new(self.boundary.dimension)
            .map(move |x| add(x, min))
            .map(|x| U::from(x))
            .zip(self.values())
    }

    /// Returns continuous slices defined by the major axis.
    pub fn rows(&self) -> impl Iterator<Item = &[T::Item]> {
        let slice = self.data.slice();
        let len = self.boundary.dimension.x as usize;
        slice
            .chunks(self.pitch.max(1))
            .map(move |slice| &slice[..len])
            .take(self.boundary.dimension.y as usize)
    }

    /// Returns all values in the array.
    pub fn values(&self) -> impl Iterator<Item = &T::Item> {
        self.rows().flatten()
    }

    /// Obtain a truncated subslice.
    ///
    /// Unlike `get`, returns a truncated result if out of bounds.
    pub fn slice(&self, boundary: impl IntoBoundary) -> Array2dRef<T::Item> {
        self.slice_internal(boundary.into_boundary()).1
    }

    /// Move the origin point of the array without affecting underlying data.
    pub fn displace(&mut self, by: impl Into<Vector2<i32>>) {
        self.boundary.min = add(self.boundary.min, by.into());
    }

    /// Move the origin point of the array without affecting underlying data.
    pub fn displaced(mut self, by: impl Into<Vector2<i32>>) -> Self {
        self.boundary.min = add(self.boundary.min, by.into());
        self
    }
}

impl<T: Array2dStorageMut> GenericArray2d<T> {
    /// Returns either a point or a slice via [`IntoBoundary`].
    ///
    /// Unlike [`slice_mut`](GenericArray2d::slice_mut), this only returns `Some` if all points are contained in the array.
    pub fn get_mut<I: Array2dIndexing<M>, M>(&mut self, point: I) -> I::ResultMut<'_, T::Item> {
        point.index_mut(self)
    }

    /// Try set a position to a value, returns `true`.
    pub fn set(&mut self, point: impl Into<Vector2<i32>>, value: T::Item) -> bool {
        if let Some(v) = point.index_mut(self) {
            *v = value;
            true
        } else {
            false
        }
    }

    /// Iterate through pairs of points and values in the array.
    pub fn iter_mut<U: From<Vector2<i32>>>(&mut self) -> impl Iterator<Item = (U, &mut T::Item)> {
        let min = self.boundary.min;
        DimensionIter::new(self.boundary.dimension)
            .map(move |x| add(x, min))
            .map(|x| U::from(x))
            .zip(self.values_mut())
    }

    /// Returns continuous slices defined by the major axis.
    pub fn rows_mut(&mut self) -> impl Iterator<Item = &mut [T::Item]> {
        let slice = self.data.slice_mut();
        let len = self.boundary.dimension.x as usize;
        slice
            .chunks_mut(self.pitch.max(1))
            .map(move |slice| &mut slice[..len])
            .take(self.boundary.dimension.x as usize)
    }

    /// Returns all values in the array.
    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut T::Item> {
        self.rows_mut().flatten()
    }

    pub fn fill(&mut self, value: T::Item)
    where
        T::Item: Clone,
    {
        for row in self.rows_mut() {
            for item in row {
                *item = value.clone();
            }
        }
    }

    /// Obtain a truncated subslice.
    ///
    /// Unlike `get`, returns a truncated result if out of bounds.
    pub fn slice_mut(&mut self, boundary: impl IntoBoundary) -> Array2dMut<T::Item> {
        self.slice_mut_internal(boundary.into_boundary()).1
    }

    /// Modify a region with another array as a "brush".
    pub fn paint<U>(
        &mut self,
        brush: &GenericArray2d<impl Array2dStorage<Item = U>>,
        at: impl Into<Vector2<i32>>,
        mut paint_fn: impl FnMut(&mut T::Item, &U),
    ) {
        let at = at.into();
        let region = Boundary {
            min: add(brush.boundary.min, at),
            dimension: brush.boundary.dimension,
        };
        let Some(intersection) = self.boundary.intersection(region) else {
            return;
        };
        let mut base1 = offset_of(intersection.min, self.boundary.min, self.pitch);
        let mut base2 = offset_of(intersection.min, region.min, brush.pitch);

        for _ in 0..intersection.dimension.y as usize {
            for i in 0..intersection.dimension.x as usize {
                paint_fn(
                    &mut self.data.slice_mut()[base1 + i],
                    &brush.data.slice()[base2 + i],
                )
            }
            base1 += self.pitch;
            base2 += brush.pitch;
        }
    }
}

impl<T: Array2dStorageOwned> GenericArray2d<T> {
    /// Initialize an array2d with a function.
    pub fn init<U: From<Vector2<i32>>>(
        boundary: impl IntoBoundary,
        mut init: impl FnMut(U) -> T::Item,
    ) -> Self {
        let boundary = boundary.into_boundary();
        let len = boundary.len();
        let mut vec = Vec::with_capacity(len);
        vec.extend(
            DimensionIter::new(boundary.dimension).map(|v| init(add(v, boundary.min).into())),
        );
        Self {
            data: T::from_vec(vec),
            boundary,
            pitch: boundary.pitch(),
        }
    }

    /// Returns the underlying buffer as a slice.
    pub fn underlying_slice(&self) -> &[T::Item] {
        &self.data.slice()[..self.len()]
    }

    /// Create an `Array2d` filled with a value.
    pub fn new_filled(boundary: impl IntoBoundary, fill: T::Item) -> Self
    where
        T::Item: Clone,
    {
        let boundary = boundary.into_boundary();
        let len = boundary.len();
        let mut vec = Vec::new();
        vec.resize(len, fill.clone());
        Self {
            data: T::from_vec(vec),
            boundary,
            pitch: boundary.pitch(),
        }
    }

    #[track_caller]
    /// Create an `Array2d` from a row major [`Vec`] as the underlying storage.
    pub fn from_vec(vec: Vec<T::Item>, boundary: impl IntoBoundary) -> Self {
        let boundary = boundary.into_boundary();
        assert!(vec.len() >= boundary.len(), "Not enough items.");
        GenericArray2d {
            data: T::from_vec(vec),
            boundary,
            pitch: boundary.pitch(),
        }
    }

    /// Create an `Array2d` with [`Default`] values.
    pub fn new(boundary: impl IntoBoundary) -> Self
    where
        T::Item: Default,
    {
        let boundary = boundary.into_boundary();
        let len = boundary.len();
        let mut vec = Vec::new();
        vec.resize_with(len, Default::default);
        Self {
            data: T::from_vec(vec),
            boundary,
            pitch: boundary.pitch(),
        }
    }

    /// Revert the array to the [`Default`] state, with size and origin point `(0, 0)`.
    pub fn clear(&mut self)
    where
        GenericArray2d<T>: Default,
    {
        *self = Default::default();
    }
}

impl<'t, T> Array2dRef<'t, T> {
    /// Create a [`Array2dRef`] by reinterpreting a row major slice.
    ///
    /// # Panics
    ///
    /// If the slice has less items than the boundary.
    pub fn from_slice(slice: &'t [T], boundary: impl IntoBoundary) -> Self {
        let boundary = boundary.into_boundary();
        assert!(slice.len() >= boundary.len(), "Not enough items.");
        Array2dRef {
            data: slice,
            boundary,
            pitch: boundary.pitch(),
        }
    }

    /// Create a [`Array2dRef`] by reinterpreting a row major slice, the offset between each row is `pitch`.
    ///
    /// # Panics
    ///
    /// * If the slice has less items than the boundary.
    /// * If the pitch is less the boundary width.
    pub fn from_slice_pitch(slice: &'t [T], boundary: impl IntoBoundary, pitch: usize) -> Self {
        let boundary = boundary.into_boundary();
        assert!(
            slice.len() >= boundary.dimension.y as usize * pitch,
            "Not enough items."
        );
        assert!(
            pitch >= boundary.pitch(),
            "Pitch must be larger than boundary."
        );
        Array2dRef {
            data: slice,
            boundary,
            pitch,
        }
    }
}

impl<'t, T> Array2dMut<'t, T> {
    /// Create a [`Array2dMut`] by reinterpreting a row major slice.
    ///
    /// # Panics
    ///
    /// If the slice has less items than the boundary.
    pub fn from_slice(slice: &'t mut [T], boundary: impl IntoBoundary) -> Self {
        let boundary = boundary.into_boundary();
        assert!(slice.len() >= boundary.len(), "Not enough items.");
        Array2dMut {
            data: slice,
            boundary,
            pitch: boundary.pitch(),
        }
    }

    /// Create a [`Array2dMut`] by reinterpreting a row major slice, the offset between each row is `pitch`.
    ///
    /// # Panics
    ///
    /// * If the slice has less items than the boundary.
    /// * If the pitch is less the boundary width.
    pub fn from_slice_pitch(slice: &'t mut [T], boundary: impl IntoBoundary, pitch: usize) -> Self {
        let boundary = boundary.into_boundary();
        assert!(
            slice.len() >= boundary.dimension.y as usize * pitch,
            "Not enough items."
        );
        assert!(
            pitch >= boundary.pitch(),
            "Pitch must be larger than boundary."
        );
        Array2dMut {
            data: slice,
            boundary,
            pitch,
        }
    }
}
