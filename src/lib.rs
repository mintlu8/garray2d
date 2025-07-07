#![doc = include_str!("../README.md")]
mod boundary;
mod impls;
mod index;
mod map;
mod resize;
mod storage;
mod util;
mod zip;
use std::fmt::Debug;
#[cfg(feature = "serde")]
mod serde;

pub use boundary::Boundary;
use boundary::IntoBoundary;
use storage::{Array2dStorage, Array2dStorageOwned};
pub use zip::Zip;

pub mod traits {
    //! Lesser used traits.
    pub use crate::boundary::IntoBoundary;
    pub use crate::index::Array2dIndexing;
    pub use crate::storage::{Array2dStorage, Array2dStorageMut, Array2dStorageOwned};
    pub use crate::zip::GenericArray2dRef;
}

/// A 2d array with generic backing storage.
#[derive(Clone, Copy)]
pub struct GenericArray2d<S: Array2dStorage> {
    data: S,
    boundary: Boundary,
    pitch: usize,
}

impl<S: Array2dStorage<Item: Debug>> Debug for GenericArray2d<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GenericArray2d")
            .field("boundary", &self.boundary)
            .field("data", &self.data.slice())
            .field("pitch", &self.pitch)
            .finish()
    }
}

/// A 2d array.
pub type Array2d<T> = GenericArray2d<Vec<T>>;

/// A readonly 2d array backed by a slice or a readonly view to an existing 2d array.
pub type Array2dRef<'t, T> = GenericArray2d<&'t [T]>;

/// A mutable 2d array backed by a mutable slice or a mutable view to an existing 2d array.
pub type Array2dMut<'t, T> = GenericArray2d<&'t mut [T]>;

impl<T> Default for Array2dRef<'_, T> {
    fn default() -> Self {
        Array2dRef {
            data: &[],
            boundary: Boundary::EMPTY,
            pitch: 0,
        }
    }
}

impl<T> Default for Array2dMut<'_, T> {
    fn default() -> Self {
        Array2dMut {
            data: &mut [],
            boundary: Boundary::EMPTY,
            pitch: 0,
        }
    }
}

impl<T: Default + Array2dStorageOwned> Default for GenericArray2d<T> {
    fn default() -> Self {
        GenericArray2d {
            data: Default::default(),
            boundary: Boundary::EMPTY,
            pitch: 0,
        }
    }
}

impl<S: Array2dStorage> IntoBoundary for &GenericArray2d<S> {
    fn into_boundary(self) -> Boundary {
        self.boundary
    }
}
