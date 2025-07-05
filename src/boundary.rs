use std::ops::{Bound, Range, RangeBounds, RangeInclusive};

use mint::Vector2;

use crate::util::*;

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

    pub fn is_empty(&self) -> bool {
        self.dimension.x == 0 || self.dimension.y == 0
    }

    pub fn len(&self) -> usize {
        (self.dimension.x * self.dimension.y) as usize
    }

    pub(crate) fn pitch(&self) -> usize {
        self.dimension.x as usize
    }

    pub fn max(&self) -> Vector2<i32> {
        sub(add(self.min, u2i(self.dimension)), [1, 1].into())
    }

    /// Returns `min + dimension` or `max + [1, 1]`.
    pub fn max_non_inclusive(&self) -> Vector2<i32> {
        add(self.min, u2i(self.dimension))
    }

    pub fn intersection(&self, other: Boundary) -> Option<Boundary> {
        let Boundary {
            min: v1,
            dimension: d1,
        } = *self;
        let Boundary {
            min: v2,
            dimension: d2,
        } = other;
        let min = vec_max(v1, v2);
        let u1 = add(v1, u2i(d1));
        let u2 = add(v2, u2i(d2));
        let max = vec_min(u1, u2);
        if max.x < min.x || max.y < min.y {
            None
        } else {
            Some(Boundary {
                min,
                dimension: i2u(sub(max, min)),
            })
        }
    }

    pub fn from_dimension(dimension: impl Into<Vector2<i32>>) -> Self {
        let dimension = i2u(dimension.into());
        Boundary {
            min: Vector2 { x: 0, y: 0 },
            dimension,
        }
    }

    pub fn min_max(min: impl Into<Vector2<i32>>, max: impl Into<Vector2<i32>>) -> Self {
        let min = min.into();
        let max = max.into();
        let dimension = i2u(add(abs(sub(max, min)), [1, 1].into()));
        Boundary { min, dimension }
    }

    pub(crate) fn min_max_non_inclusive(
        min: impl Into<Vector2<i32>>,
        max: impl Into<Vector2<i32>>,
    ) -> Self {
        let min = min.into();
        let max = max.into();
        let dimension = i2u(abs(sub(max, min)));
        Boundary { min, dimension }
    }

    pub fn min_dim(min: impl Into<Vector2<i32>>, dimension: impl Into<Vector2<i32>>) -> Self {
        let min = min.into();
        let dimension = i2u(abs(dimension.into()));
        Boundary { min, dimension }
    }

    pub fn center_hdim(center: impl Into<Vector2<i32>>, half_dim: impl Into<Vector2<i32>>) -> Self {
        let center = center.into();
        let half_dim = abs(half_dim.into());
        let min = sub(center, half_dim);
        let dimension = i2u(add(add(half_dim, half_dim), [1, 1].into()));
        Boundary { min, dimension }
    }

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
            Bound::Unbounded => i32::MAX,
        };
        let max_y = match y.end_bound() {
            Bound::Included(v) => *v,
            Bound::Excluded(v) => *v - 1,
            Bound::Unbounded => i32::MAX,
        };
        Boundary::min_max([min_x, min_y], [max_x, max_y])
    }

    pub fn contains(&self, position: impl Into<Vector2<i32>>) -> bool {
        let position = position.into();
        position.x >= self.min.x
            && position.y >= self.min.y
            && position.x < self.min.x + self.dimension.x as i32
            && position.y < self.min.y + self.dimension.y as i32
    }
}

/// Types that can be used as [`Boundary`],
///
/// implemented on `Boundary`, `Range<impl Into<Vector2<i32>>` (i.e. `[0, 0]..[4, 5]`),
/// `RangeInclusive<impl Into<Vector2<i32>>` (i.e. `[0, 0]..=[4, 5]`) and
/// `(impl RangeBounds<i32>, impl RangeBounds<i32>)` (i.e. `(0..4, 0..=5)`)
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
