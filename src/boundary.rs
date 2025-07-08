use std::ops::{
    Bound, Range, RangeBounds, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
};

use mint::Vector2;

use crate::util::*;

const MIN: Vector2<i32> = Vector2 {
    x: i32::MIN,
    y: i32::MIN,
};

const MAX: Vector2<i32> = Vector2 {
    x: i32::MAX - 1,
    y: i32::MAX - 1,
};

/// Area occupied by a 2d array.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Boundary {
    pub min: Vector2<i32>,
    pub dimension: Vector2<u32>,
}

impl Default for Boundary {
    fn default() -> Self {
        Self {
            min: Vector2 { x: 0, y: 0 },
            dimension: Vector2 { x: 0, y: 0 },
        }
    }
}

impl Boundary {
    pub const EMPTY: Self = Self {
        min: Vector2 { x: 0, y: 0 },
        dimension: Vector2 { x: 0, y: 0 },
    };

    pub const ALL: Self = Self {
        min: Vector2 {
            x: i32::MIN,
            y: i32::MIN,
        },
        dimension: Vector2 {
            x: u32::MAX,
            y: u32::MAX,
        },
    };

    /// Returns true if contains 0 points.
    pub fn is_empty(&self) -> bool {
        self.dimension.x == 0 || self.dimension.y == 0
    }

    /// Returns the length of the underlying vector.
    pub fn len(&self) -> usize {
        (self.dimension.x * self.dimension.y) as usize
    }

    /// Returns the major axis.
    pub(crate) fn pitch(&self) -> usize {
        self.dimension.x as usize
    }

    /// Returns the maximum point.
    pub fn max(&self) -> Vector2<i32> {
        sub(addu(self.min, self.dimension), [1, 1].into())
    }

    /// Returns `min + dimension` or `max + [1, 1]`.
    pub fn max_non_inclusive(&self) -> Vector2<i32> {
        addu(self.min, self.dimension)
    }

    pub fn intersection(&self, other: Boundary) -> Option<Boundary> {
        let min = vec_max(self.min, other.min);
        let u1 = self.max_non_inclusive();
        let u2 = other.max_non_inclusive();
        let max = vec_min(u1, u2);
        if max.x < min.x || max.y < min.y {
            None
        } else {
            Some(Boundary::min_max_non_inclusive(min, max))
        }
    }

    /// Returns boundary of a point with dimension `[1, 1]`.
    pub fn from_point(point: impl Into<Vector2<i32>>) -> Self {
        Boundary {
            min: point.into(),
            dimension: Vector2 { x: 1, y: 1 },
        }
    }

    /// Returns boundary of a conventional 2d array starting from `[0, 0]`.
    pub fn from_dimension(dimension: impl Into<Vector2<i32>>) -> Self {
        let dimension = i2u(dimension.into());
        Boundary {
            min: Vector2 { x: 0, y: 0 },
            dimension,
        }
    }

    /// Returns boundary from a minimum and maximum point.
    pub fn min_max(min: impl Into<Vector2<i32>>, max: impl Into<Vector2<i32>>) -> Self {
        let min = min.into();
        let max = max.into();
        if min == MIN && max == MAX {
            // since length is u32::MAX + 1
            Boundary::ALL
        } else {
            let dimension = Vector2 {
                x: max.x.wrapping_sub(min.x).wrapping_add(1) as u32,
                y: max.y.wrapping_sub(min.y).wrapping_add(1) as u32,
            };
            Boundary { min, dimension }
        }
    }

    /// Returns boundary from a minimum and a non-inclusive maximum point.
    ///
    /// # Note
    ///
    /// `i32::MAX` is not allowed, use `i32::MAX - 1` instead.
    pub(crate) fn min_max_non_inclusive(
        min: impl Into<Vector2<i32>>,
        max: impl Into<Vector2<i32>>,
    ) -> Self {
        let min = min.into();
        let max = max.into();
        let dimension = Vector2 {
            x: max.x.wrapping_sub(min.x) as u32,
            y: max.y.wrapping_sub(min.y) as u32,
        };
        Boundary { min, dimension }
    }

    /// Returns boundary from a minimum point and a dimension.
    pub fn min_dim(min: impl Into<Vector2<i32>>, dimension: impl Into<Vector2<i32>>) -> Self {
        let min = min.into();
        let dimension = i2u(abs(dimension.into()));
        Boundary { min, dimension }
    }

    /// Returns boundary from a center point and half dimension.
    pub fn center_hdim(center: impl Into<Vector2<i32>>, half_dim: impl Into<Vector2<i32>>) -> Self {
        let center = center.into();
        let half_dim = abs(half_dim.into());
        let min = sub(center, half_dim);
        let dimension = i2u(add(add(half_dim, half_dim), [1, 1].into()));
        Boundary { min, dimension }
    }

    /// Returns boundary from 2 ranges.
    pub fn xy(x: impl RangeBounds<i32>, y: impl RangeBounds<i32>) -> Self {
        let min_x = match x.start_bound() {
            Bound::Included(v) => *v,
            Bound::Excluded(v) => *v + 1,
            Bound::Unbounded => i32::MIN,
        };
        let min_y = match y.start_bound() {
            Bound::Included(v) => *v,
            Bound::Excluded(v) => *v + 1,
            Bound::Unbounded => i32::MIN,
        };
        let max_x = match x.end_bound() {
            Bound::Included(v) => *v,
            Bound::Excluded(v) => *v - 1,
            Bound::Unbounded => i32::MAX - 1,
        };
        let max_y = match y.end_bound() {
            Bound::Included(v) => *v,
            Bound::Excluded(v) => *v - 1,
            Bound::Unbounded => i32::MAX - 1,
        };
        Boundary::min_max([min_x, min_y], [max_x, max_y])
    }

    /// Move the boundary.
    pub fn displace(&mut self, by: impl Into<Vector2<i32>>) {
        self.min = add(self.min, by.into())
    }

    /// Move the boundary.
    pub fn displace_by(&self, by: impl Into<Vector2<i32>>) -> Boundary {
        let mut result = *self;
        result.displace(by);
        result
    }

    /// Increase dimension both horizontally and vertically.
    ///
    /// For example expanding `[0, 0]..=[0, 0]` by `[2, 1]`
    /// results in `[-2, -1]..=[2, 1]`.
    pub fn expand(&mut self, by: impl Into<Vector2<i32>>) {
        let by = by.into();
        self.min = sub(self.min, by);
        self.dimension = i2u(add(u2i(self.dimension), by));
    }

    /// Increase dimension both horizontally and vertically.
    ///
    /// For example expanding `[0, 0]..=[0, 0]` by `[2, 1]`
    /// results in `[-2, -1]..=[2, 1]`.
    pub fn expand_by(&self, by: impl Into<Vector2<i32>>) -> Boundary {
        let mut result = *self;
        result.expand(by);
        result
    }

    /// Returns `true` if contains a point.
    pub fn contains(&self, position: impl Into<Vector2<i32>>) -> bool {
        let position = position.into();
        position.x >= self.min.x
            && position.y >= self.min.y
            && position.x < self.min.x.wrapping_add(self.dimension.x as i32)
            && position.y < self.min.y.wrapping_add(self.dimension.y as i32)
    }

    /// Iterate through all points in the boundary.
    pub fn iter<T: From<Vector2<i32>>>(&self) -> impl Iterator<Item = T> + 'static + use<T> {
        let min = self.min;
        DimensionIter::new(self.dimension).map(move |x| add(x, min).into())
    }
}

/// Types that can be used as [`Boundary`].
pub trait IntoBoundary {
    fn into_boundary(self) -> Boundary;
}

impl IntoBoundary for Boundary {
    fn into_boundary(self) -> Boundary {
        self
    }
}

impl<T: IntoBoundary + Copy> IntoBoundary for &T {
    fn into_boundary(self) -> Boundary {
        (*self).into_boundary()
    }
}

impl<U: Into<Vector2<i32>>> IntoBoundary for Range<U> {
    fn into_boundary(self) -> Boundary {
        Boundary::min_max_non_inclusive(self.start, self.end)
    }
}

impl<U: Into<Vector2<i32>>> IntoBoundary for RangeInclusive<U> {
    fn into_boundary(self) -> Boundary {
        let (min, max) = self.into_inner();
        Boundary::min_max(min, max)
    }
}

impl<U: Into<Vector2<i32>>> IntoBoundary for RangeFrom<U> {
    fn into_boundary(self) -> Boundary {
        let min = self.start.into();
        Boundary::min_max(min, MAX)
    }
}

impl<U: Into<Vector2<i32>>> IntoBoundary for RangeTo<U> {
    fn into_boundary(self) -> Boundary {
        let max: Vector2<i32> = self.end.into();
        Boundary::min_max_non_inclusive(MIN, max)
    }
}

impl<U: Into<Vector2<i32>>> IntoBoundary for RangeToInclusive<U> {
    fn into_boundary(self) -> Boundary {
        let max: Vector2<i32> = self.end.into();
        Boundary::min_max(MIN, max)
    }
}

impl IntoBoundary for RangeFull {
    fn into_boundary(self) -> Boundary {
        Boundary::ALL
    }
}

impl<A: RangeBounds<i32>, B: RangeBounds<i32>> IntoBoundary for (A, B) {
    fn into_boundary(self) -> Boundary {
        Boundary::xy(self.0, self.1)
    }
}

impl IntoBoundary for [u32; 2] {
    fn into_boundary(self) -> Boundary {
        Boundary {
            min: Vector2 { x: 0, y: 0 },
            dimension: self.into(),
        }
    }
}
