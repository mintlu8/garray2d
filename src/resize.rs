//! Implementation for map/dictionary like methods.

use mint::Vector2;

use crate::{
    GenericArray2d,
    boundary::{Boundary, IntoBoundary},
    storage::{Array2dStorage, Array2dStorageOwned},
    util::*,
};

impl<T: Array2dStorageOwned<Item: Default>> GenericArray2d<T> {
    fn downsize(&mut self, boundary: Boundary) {
        let mut base1 = 0;
        let mut base2 = offset_of(boundary.min, self.boundary.min, self.pitch);
        let slice = self.data.slice_mut();
        for _ in 0..boundary.dimension.y {
            move_within(slice, base2, base1, boundary.pitch());
            base1 += boundary.pitch();
            base2 += self.pitch;
        }
        let len = boundary.len();
        slice[len..].fill_with(Default::default)
    }

    fn upsize(&mut self, from: Boundary, to: Boundary) {
        let h = from.dimension.y as usize;
        let mut base1 = from.pitch() * h;
        let mut base2 = offset_of(from.min, to.min, to.pitch()) + to.pitch() * h;
        let slice = self.data.slice_mut();
        for _ in 0..h {
            base1 -= from.pitch();
            base2 -= to.pitch();
            move_within(slice, base1, base2, from.pitch());
        }
    }

    /// Resize the array and initialize with [`Default::default`].
    pub fn resize(&mut self, boundary: impl IntoBoundary) {
        let boundary = boundary.into_boundary();

        if self.boundary == boundary {
            return;
        }
        let size = boundary.len();
        if size > self.data.slice().len() {
            self.data.vec_mut().resize_with(size, Default::default);
        }

        let Some(intersection) = self.boundary.intersection(boundary) else {
            self.data.vec_mut().fill_with(Default::default);
            self.boundary = boundary;
            self.pitch = boundary.pitch();
            return;
        };

        // Downsizing is ordered to avoid use after move,
        // then we can clear unused items.
        if intersection != self.boundary {
            self.downsize(intersection);
        }

        // Upsizing is ordered to avoid use after move.
        if intersection != boundary {
            self.upsize(intersection, boundary);
        }

        self.boundary = boundary;
        self.pitch = boundary.pitch();
    }

    /// Insert a point into an array and potentially expanding the size with [`Default`] values.
    pub fn insert(&mut self, position: impl Into<Vector2<i32>>, value: T::Item) {
        let position: Vector2<i32> = position.into();
        if self.contains(position) {
            if let Some(v) = self.get_mut(position) {
                *v = value;
            }
        } else {
            let mut min = self.boundary.min;
            let mut dimension = self.boundary.dimension;
            let max: Vector2<i32> = self.max_point();
            if position.x < min.x {
                min.x = position.x;
            } else if position.x > max.x {
                dimension.x += (position.x - max.x) as u32;
            }
            if position.y < min.y {
                min.y = position.y;
            } else if position.y > max.y {
                dimension.y += (position.y - max.y) as u32;
            }
            self.resize(Boundary { min, dimension });
        }
    }

    /// Extend the array to cover a boundary, then insert points into the array.
    /// Points outside of the new boundary will be discarded.
    ///
    /// # Returns
    ///
    /// `true` if no points are discarded.
    pub fn extend<U: Into<Vector2<i32>>>(
        &mut self,
        boundary: impl IntoBoundary,
        positions: impl IntoIterator<Item = (U, T::Item)>,
    ) -> bool {
        let mut no_discards = true;
        let Boundary { min, dimension } = boundary.into_boundary();
        let max = vec_max(
            add(self.boundary.min, u2i(self.boundary.dimension)),
            add(min, u2i(dimension)),
        );
        let min = vec_min(self.boundary.min, min);
        let dimension = i2u(sub(max, min));

        self.resize(Boundary { min, dimension });

        for (position, item) in positions {
            if let Some(v) = self.get_mut(position) {
                *v = item;
            } else {
                no_discards = false
            }
        }
        no_discards
    }

    /// Extend the array to cover both array's boundaries and copy the other array into this array.
    pub fn merge<U: Array2dStorage<Item = T::Item>>(&mut self, array: &GenericArray2d<U>)
    where
        T::Item: Clone,
    {
        let min = vec_min(self.boundary.min, array.boundary.min);
        let max = vec_max(
            self.boundary.max_non_inclusive(),
            array.boundary.max_non_inclusive(),
        );
        let dimension = i2u(sub(max, min));

        self.resize(Boundary { min, dimension });

        self.paint(array, [0, 0], |source, incoming| *source = incoming.clone());
    }
}
