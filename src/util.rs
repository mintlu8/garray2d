use std::mem;

use mint::Vector2;

pub fn add(left: Vector2<i32>, right: Vector2<i32>) -> Vector2<i32> {
    Vector2 {
        x: left.x + right.x,
        y: left.y + right.y,
    }
}

pub fn sub(left: Vector2<i32>, right: Vector2<i32>) -> Vector2<i32> {
    Vector2 {
        x: left.x - right.x,
        y: left.y - right.y,
    }
}

pub fn vec_min(left: Vector2<i32>, right: Vector2<i32>) -> Vector2<i32> {
    Vector2 {
        x: left.x.min(right.x),
        y: left.y.min(right.y),
    }
}

pub fn vec_max(left: Vector2<i32>, right: Vector2<i32>) -> Vector2<i32> {
    Vector2 {
        x: left.x.max(right.x),
        y: left.y.max(right.y),
    }
}

pub fn u2i(v: Vector2<u32>) -> Vector2<i32> {
    Vector2 {
        x: v.x as i32,
        y: v.y as i32,
    }
}

pub fn i2u(v: Vector2<i32>) -> Vector2<u32> {
    Vector2 {
        x: v.x.max(0) as u32,
        y: v.y.max(0) as u32,
    }
}

pub fn abs(v: Vector2<i32>) -> Vector2<i32> {
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
