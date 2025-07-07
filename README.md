# garray2d

[![Crates.io](https://img.shields.io/crates/v/garray2d.svg)](https://crates.io/crates/garray2d)
[![Docs](https://docs.rs/garray2d/badge.svg)](https://docs.rs/garray2d/latest/garray2d/)

Game development focused 2d array with signed index and offset support.

## Design

`Array2d` represents a rectangular region in a tile map,
with non-represented regions considered `Default::default`.
This allows us to implement functions like `resize` that are geometric
instead of reinterpreting the underlying bytes.

`Array2d` is row major, which makes it useful when working with gpu textures.
There is currently no direct support for column major arrays.

## Usage

`Array2d` is analogous to a `Vec` and can be created either via a function initializer
similar to `array::from_fn` or from a `Vec` and a dimension.

`Array2dRef` and `Array2dMut` can be created either by slicing an existing 2d array
using `get` or `slice` or by reinterpreting a slice as a 2d array.

To combine multiple arrays, use `zip` if dimension is the same, `paint` if you
do not care about overflows and `merge` if you do.

## Core Traits

We use a few traits to make your life easier when using this crate,
here are the common implementors of these types.

* `IntoBoundary`

`IntoBoundary` means convertible to a 2d rectangle, types that implement it are

| type | example(s) | note |
| - | - | - |
| `Boundary` | `Boundary::min_max([0, 0], [5, 8])` | |
| `[u32; 2]` | `[5, 8]` | Represents `[0, 0]..[5, 8]` |
| `impl RangeBounds<Into<Vector2<i32>>>` | `[-1, -1]..[1, 1]` | Only for `std` types |
| `(impl RangeBounds<i32>, impl RangeBounds<i32>)` | `(-1..=1, -1..1)` | |

* `Into<Vector2<i32>>`

The core `mint` trait that's implemented by `[i32; 2]` and types like `glam`'s `IVec2` or `nalgebra`'s `Vector2<i32>`.

* `Array2dIndexing`

For the `get` function, types that implement it are

| type | example(s) | note |
| - | - | - |
| `impl Into<Vector2<i32>>` | `[-1, -1]` | Returns a point. |
| `impl IntoBoundary` | `Boundary::min_max([0, 0], [1024, 768])` | Returns a slice. |

## Usage with Math Libraries

This crate uses `mint` to interop with math crates like `glam` or `nalgebra`.
In `glam` or `bevy_math` for example, you might want enable the `mint` feature.
The core trait `Into<Vector2<i32>>` is implemented by types like `glam`'s `IVec2`.
Additionally, `[i32; 2]` always implements this trait and should be the easiest way to
create a quick constant.

```rust
array.get([1, 2])
```

## License

License under either of

Apache License, Version 2.0 (LICENSE-APACHE or <http://www.apache.org/licenses/LICENSE-2.0>)
MIT license (LICENSE-MIT or <http://opensource.org/licenses/MIT>)
at your option.

## Contribution

Contributions are welcome!

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
