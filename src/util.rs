use std::mem;

use mint::Vector2;

use crate::{
    Array2dMut, Array2dRef, Boundary, GenericArray2d,
    traits::{Array2dStorage, Array2dStorageMut},
};

pub(crate) const ZERO: Vector2<i32> = Vector2 { x: 0, y: 0 };
pub(crate) const ONE: Vector2<i32> = Vector2 { x: 1, y: 1 };

#[track_caller]
pub(crate) const fn add(left: Vector2<i32>, right: Vector2<i32>) -> Vector2<i32> {
    Vector2 {
        x: left.x + right.x,
        y: left.y + right.y,
    }
}

#[track_caller]
pub(crate) const fn addu(left: Vector2<i32>, right: Vector2<u32>) -> Vector2<i32> {
    Vector2 {
        x: left.x.wrapping_add(right.x as i32),
        y: left.y.wrapping_add(right.y as i32),
    }
}

#[track_caller]
pub(crate) const fn sub(left: Vector2<i32>, right: Vector2<i32>) -> Vector2<i32> {
    Vector2 {
        x: left.x - right.x,
        y: left.y - right.y,
    }
}

#[track_caller]
pub(crate) fn vec_min(left: Vector2<i32>, right: Vector2<i32>) -> Vector2<i32> {
    Vector2 {
        x: left.x.min(right.x),
        y: left.y.min(right.y),
    }
}

#[track_caller]
pub(crate) fn vec_max(left: Vector2<i32>, right: Vector2<i32>) -> Vector2<i32> {
    Vector2 {
        x: left.x.max(right.x),
        y: left.y.max(right.y),
    }
}

#[track_caller]
pub(crate) const fn u2i(v: Vector2<u32>) -> Vector2<i32> {
    Vector2 {
        x: v.x as i32,
        y: v.y as i32,
    }
}

#[track_caller]
pub(crate) const fn i2u(v: Vector2<i32>) -> Vector2<u32> {
    Vector2 {
        x: if v.x > 0 { v.x as u32 } else { 0 },
        y: if v.y > 0 { v.y as u32 } else { 0 },
    }
}

#[track_caller]
pub(crate) const fn abs(v: Vector2<i32>) -> Vector2<i32> {
    Vector2 {
        x: v.x.abs(),
        y: v.y.abs(),
    }
}

#[inline]
#[track_caller]
pub const fn offset_of(pos: Vector2<i32>, origin: Vector2<i32>, pitch: usize) -> usize {
    let pos = sub(pos, origin);
    pos.y as usize * pitch + pos.x as usize
}

#[inline]
#[track_caller]
pub fn move_within<T: Default>(slice: &mut [T], from: usize, to: usize, len: usize) {
    if from == to {
    } else if to < from || from + len <= to {
        for i in 0..len {
            slice[to + i] = mem::take(&mut slice[from + i]);
        }
    } else {
        for i in (0..len).rev() {
            slice[to + i] = mem::take(&mut slice[from + i]);
        }
    }
}

pub struct DimensionIter {
    position: Vector2<u32>,
    dimension: Vector2<u32>,
}

impl DimensionIter {
    pub fn new(dimension: Vector2<u32>) -> Self {
        DimensionIter {
            position: Vector2 { x: 0, y: 0 },
            dimension,
        }
    }
}

impl Iterator for DimensionIter {
    type Item = Vector2<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.position;
        if self.position.y >= self.dimension.y {
            return None;
        }
        self.position.x += 1;
        if self.position.x >= self.dimension.x {
            self.position.x = 0;
            self.position.y += 1;
        }
        Some(u2i(out))
    }
}

pub struct BorderIter {
    edge_no: u32,
    position: u32,
    // dimension - 1
    d_1: Vector2<u32>,
}

impl BorderIter {
    pub fn new(dimension: Vector2<u32>) -> Self {
        if dimension.x == 0 || dimension.y == 0 {
            BorderIter {
                edge_no: 5,
                position: 0,
                d_1: Vector2 { x: 0, y: 0 },
            }
        } else if dimension.y == 1 {
            BorderIter {
                edge_no: 0,
                position: 0,
                d_1: Vector2 {
                    x: dimension.x,
                    y: 0,
                },
            }
        } else if dimension.x == 1 {
            BorderIter {
                edge_no: 1,
                position: 0,
                d_1: Vector2 {
                    x: 0,
                    y: dimension.y,
                },
            }
        } else {
            BorderIter {
                edge_no: 0,
                position: 0,
                d_1: Vector2 {
                    x: dimension.x - 1,
                    y: dimension.y - 1,
                },
            }
        }
    }
}

impl Iterator for BorderIter {
    type Item = Vector2<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        let (result, limit) = match self.edge_no {
            0 => (
                Vector2 {
                    x: self.position,
                    y: 0,
                },
                self.d_1.x,
            ),
            1 => (
                Vector2 {
                    x: self.d_1.x,
                    y: self.position,
                },
                self.d_1.y,
            ),
            2 => (
                Vector2 {
                    x: self.d_1.x - self.position,
                    y: self.d_1.y,
                },
                self.d_1.x,
            ),
            3 => (
                Vector2 {
                    x: 0,
                    y: self.d_1.y - self.position,
                },
                self.d_1.y,
            ),
            _ => return None,
        };
        self.position += 1;
        if self.position >= limit {
            self.position = 0;
            self.edge_no += 1;
            if self.d_1.x == 0 || self.d_1.y == 0 {
                self.edge_no = 5;
            }
        }
        Some(u2i(result))
    }
}

pub(crate) struct IterOwned<T> {
    pub iter: T,
    pub position: Vector2<u32>,
    pub dimension: Vector2<u32>,
    pub pitch: u32,
}

impl<T: Iterator> Iterator for IterOwned<T> {
    type Item = (Vector2<i32>, T::Item);

    fn next(&mut self) -> Option<Self::Item> {
        if self.position.y >= self.dimension.y {
            return None;
        }
        let out = (u2i(self.position), self.iter.next()?);
        self.position.x += 1;
        if self.position.x >= self.dimension.x {
            self.position.x = 0;
            self.position.y += 1;
            for _ in self.dimension.x..self.pitch {
                let _ = self.iter.next();
            }
        }
        Some(out)
    }
}

impl<T: Array2dStorage> GenericArray2d<T> {
    #[inline]
    pub(crate) fn index_internal(&self, point: Vector2<i32>) -> Option<usize> {
        let x = point.x - self.boundary.min.x;
        let y = point.y - self.boundary.min.y;
        if x < 0 || x >= self.boundary.dimension.x as i32 {
            return None;
        }
        if y < 0 || y >= self.boundary.dimension.y as i32 {
            return None;
        }
        Some(y as usize * self.pitch + x as usize)
    }

    pub(crate) fn slice_internal(&self, input: Boundary) -> (bool, Array2dRef<'_, T::Item>) {
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
    pub(crate) fn slice_mut_internal(
        &mut self,
        input: Boundary,
    ) -> (bool, Array2dMut<'_, T::Item>) {
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

#[cfg(test)]
mod test {
    use crate::util::BorderIter;

    #[test]
    pub fn test_iter_border() {
        fn v(iter: BorderIter) -> Vec<[i32; 2]> {
            iter.map(Into::into).collect()
        }
        assert_eq!(v(BorderIter::new([0, 0].into())), Vec::<[i32; 2]>::new());
        assert_eq!(v(BorderIter::new([1, 1].into())), vec![[0, 0]]);
        assert_eq!(
            v(BorderIter::new([1, 4].into())),
            vec![[0, 0], [0, 1], [0, 2], [0, 3]]
        );
        assert_eq!(
            v(BorderIter::new([4, 1].into())),
            vec![[0, 0], [1, 0], [2, 0], [3, 0]]
        );
        assert_eq!(
            v(BorderIter::new([2, 2].into())),
            vec![[0, 0], [1, 0], [1, 1], [0, 1]]
        );
        assert_eq!(
            v(BorderIter::new([3, 4].into())),
            vec![
                [0, 0],
                [1, 0],
                [2, 0],
                [2, 1],
                [2, 2],
                [2, 3],
                [1, 3],
                [0, 3],
                [0, 2],
                [0, 1],
            ]
        );
    }
}
