use std::mem;

use mint::Vector2;

#[track_caller]
pub(crate) fn add(left: Vector2<i32>, right: Vector2<i32>) -> Vector2<i32> {
    Vector2 {
        x: left.x + right.x,
        y: left.y + right.y,
    }
}

#[track_caller]
pub(crate) fn addu(left: Vector2<i32>, right: Vector2<u32>) -> Vector2<i32> {
    Vector2 {
        x: left.x.wrapping_add(right.x as i32),
        y: left.y.wrapping_add(right.y as i32),
    }
}

#[track_caller]
pub(crate) fn sub(left: Vector2<i32>, right: Vector2<i32>) -> Vector2<i32> {
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
pub(crate) fn u2i(v: Vector2<u32>) -> Vector2<i32> {
    Vector2 {
        x: v.x as i32,
        y: v.y as i32,
    }
}

#[track_caller]
pub(crate) fn i2u(v: Vector2<i32>) -> Vector2<u32> {
    Vector2 {
        x: v.x.max(0) as u32,
        y: v.y.max(0) as u32,
    }
}

#[track_caller]
pub(crate) fn abs(v: Vector2<i32>) -> Vector2<i32> {
    Vector2 {
        x: v.x.abs(),
        y: v.y.abs(),
    }
}

#[inline]
#[track_caller]
pub fn offset_of(pos: Vector2<i32>, origin: Vector2<i32>, pitch: usize) -> usize {
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
