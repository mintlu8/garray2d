use std::fmt::Debug;

use garray2d::{Array2d, Boundary};
use glam::IVec2;

#[track_caller]
fn iter_eq<T: PartialEq + Debug>(a: impl IntoIterator<Item = T>, b: impl IntoIterator<Item = T>) {
    let a = a.into_iter();
    let mut b = b.into_iter();
    for v in a {
        assert_eq!(Some(v), b.next());
    }
    assert!(b.next().is_none())
}

#[test]
pub fn boundary() {
    iter_eq(Boundary::EMPTY.iter::<[i32; 2]>(), []);

    iter_eq(
        Boundary::min_max([1, 1], [2, 3]).iter::<[i32; 2]>(),
        [[1, 1], [2, 1], [1, 2], [2, 2], [1, 3], [2, 3]],
    );
}

#[test]
pub fn create() {
    let arr = Array2d::init((-1..2, -1..3), |v: IVec2| v);
    assert_eq!(arr.len(), 12);
    assert_eq!(arr.width(), 3);
    assert_eq!(arr.height(), 4);

    let arr = Array2d::init((-1..=2, -1..=3), |v: IVec2| v);
    assert_eq!(arr.len(), 20);
    assert_eq!(arr.width(), 4);
    assert_eq!(arr.height(), 5);

    let arr = Array2d::init([1, 4]..[7, 12], |v: IVec2| v);
    assert_eq!(arr.len(), 48);
    assert_eq!(arr.width(), 6);
    assert_eq!(arr.height(), 8);

    let arr = Array2d::init([1, 4]..=[7, 12], |v: IVec2| v);
    assert_eq!(arr.len(), 63);
    assert_eq!(arr.width(), 7);
    assert_eq!(arr.height(), 9);

    let arr = Array2d::init(
        Boundary::min_max(IVec2::new(1, 2), IVec2::new(4, 3)),
        |v: IVec2| v,
    );
    assert_eq!(arr.len(), 8);
    assert_eq!(arr.width(), 4);
    assert_eq!(arr.height(), 2);

    let arr = Array2d::init(
        Boundary::center_hdim(IVec2::new(0, 0), IVec2::new(3, 4)),
        |v: IVec2| v,
    );
    assert_eq!(arr.len(), 63);
    assert_eq!(arr.width(), 7);
    assert_eq!(arr.height(), 9);

    let arr = Array2d::init(
        Boundary::min_dim(IVec2::new(0, 0), IVec2::new(1, 4)),
        |v: IVec2| v,
    );
    assert_eq!(arr.len(), 4);
    assert_eq!(arr.width(), 1);
    assert_eq!(arr.height(), 4);
}

#[test]
pub fn getters() {
    let arr = Array2d::init((0..=0, 0..5), |v: IVec2| v.y);
    assert_eq!(arr.get(IVec2::new(0, -1)).copied(), None);
    assert_eq!(arr.get(IVec2::new(0, 0)).copied(), Some(0));
    assert_eq!(arr.get(IVec2::new(0, 1)).copied(), Some(1));
    assert_eq!(arr.get(IVec2::new(0, 2)).copied(), Some(2));
    assert_eq!(arr.get(IVec2::new(0, 3)).copied(), Some(3));
    assert_eq!(arr.get(IVec2::new(0, 4)).copied(), Some(4));
    assert_eq!(arr.get(IVec2::new(0, 5)).copied(), None);

    assert_eq!(arr.get(IVec2::new(-1, 0)).copied(), None);
    assert_eq!(arr.get(IVec2::new(-1, 1)).copied(), None);
    assert_eq!(arr.get(IVec2::new(-1, 2)).copied(), None);
    assert_eq!(arr.get(IVec2::new(-1, 3)).copied(), None);
    assert_eq!(arr.get(IVec2::new(-1, 4)).copied(), None);

    assert_eq!(arr.get(IVec2::new(1, 0)).copied(), None);
    assert_eq!(arr.get(IVec2::new(1, 1)).copied(), None);
    assert_eq!(arr.get(IVec2::new(1, 2)).copied(), None);
    assert_eq!(arr.get(IVec2::new(1, 3)).copied(), None);
    assert_eq!(arr.get(IVec2::new(1, 4)).copied(), None);

    let arr = Array2d::init(Boundary::min_max([-1, -1], [1, 1]), |v: IVec2| {
        v.x * 7 + v.y * 5
    });
    assert_eq!(arr.len(), 9);
    assert_eq!(arr.get(IVec2::new(-1, -1)).copied(), Some(-12));
    assert_eq!(arr.get(IVec2::new(-1, 0)).copied(), Some(-7));
    assert_eq!(arr.get(IVec2::new(-1, 1)).copied(), Some(-2));
    assert_eq!(arr.get(IVec2::new(0, 0)).copied(), Some(0));
    assert_eq!(arr.get(IVec2::new(1, -1)).copied(), Some(2));
    assert_eq!(arr.get(IVec2::new(1, 1)).copied(), Some(12));
    assert_eq!(arr.get(IVec2::new(-2, 0)).copied(), None);
    assert_eq!(arr.get(IVec2::new(2, 0)).copied(), None);
    assert_eq!(arr.get(IVec2::new(0, -2)).copied(), None);
    assert_eq!(arr.get(IVec2::new(0, 2)).copied(), None);

    let arr = Array2d::init((0..3, 0..2), |v: IVec2| v);
    assert_eq!(arr.len(), 6);

    assert_eq!(arr.get(IVec2::new(2, 1)).copied(), Some(IVec2::new(2, 1)));
    assert_eq!(arr.get(IVec2::new(0, 0)).copied(), Some(IVec2::new(0, 0)));
    assert_eq!(arr.get(IVec2::new(-1, 2)).copied(), None);
    assert_eq!(arr.get(IVec2::new(1, 2)).copied(), None);

    let arr = Array2d::init(Boundary::min_max([-1, -1], [1, 1]), |v: IVec2| {
        v.x * 7 + v.y * 5
    });
    let slice_1 = arr.get((0..=1, 0..=1)).unwrap();
    iter_eq(
        slice_1.iter::<IVec2>(),
        [
            (IVec2::new(0, 0), &0),
            (IVec2::new(1, 0), &7),
            (IVec2::new(0, 1), &5),
            (IVec2::new(1, 1), &12),
        ],
    );

    assert!(arr.get((0..=2, 0..=2)).is_none());

    let arr = Array2d::init(Boundary::min_max([0, 0], [8, 5]), |v: IVec2| {
        v.x * 7 + v.y * 5
    });
    let slice_1 = arr.get([4, 4]..=[7, 5]).unwrap();
    iter_eq(
        slice_1.iter::<IVec2>(),
        [
            (IVec2::new(4, 4), &48),
            (IVec2::new(5, 4), &55),
            (IVec2::new(6, 4), &62),
            (IVec2::new(7, 4), &69),
            (IVec2::new(4, 5), &53),
            (IVec2::new(5, 5), &60),
            (IVec2::new(6, 5), &67),
            (IVec2::new(7, 5), &74),
        ],
    );
}

#[test]
pub fn resize() {
    let mut arr = Array2d::init((0..=0, 0..5), |v: IVec2| v.y);
    arr.resize((-1..=2, 0..6));
    iter_eq(
        arr.rows(),
        [
            &[0, 0, 0, 0],
            &[0, 1, 0, 0],
            &[0, 2, 0, 0],
            &[0, 3, 0, 0],
            &[0, 4, 0, 0],
            &[0, 0, 0, 0],
        ] as [&[_]; 6],
    );

    let mut arr = Array2d::init((0..5, 0..=0), |v: IVec2| v.x);
    arr.resize((-1..6, -1..=2));
    iter_eq(
        arr.rows(),
        [
            &[0, 0, 0, 0, 0, 0, 0],
            &[0, 0, 1, 2, 3, 4, 0],
            &[0, 0, 0, 0, 0, 0, 0],
            &[0, 0, 0, 0, 0, 0, 0],
        ] as [&[_]; 4],
    );

    let mut arr = Array2d::init((0..=5, 0..=1), |v: IVec2| v.x);
    arr.resize((1..=3, -2..=1));

    iter_eq(
        arr.rows(),
        [&[0, 0, 0], &[0, 0, 0], &[1, 2, 3], &[1, 2, 3]] as [&[_]; 4],
    );

    let mut arr = Array2d::init((0..=1, 0..=5), |v: IVec2| v.y);
    arr.resize((-2..=1, 1..=3));

    iter_eq(
        arr.rows(),
        [&[0, 0, 1, 1], &[0, 0, 2, 2], &[0, 0, 3, 3]] as [&[_]; 3],
    );

    let mut arr = Array2d::init((1..=5, 0..=2), |v: IVec2| v.x);
    arr.resize((2..=4, 2..=4));

    iter_eq(
        arr.rows(),
        [&[2, 3, 4], &[0, 0, 0], &[0, 0, 0]] as [&[_]; 3],
    );

    let mut arr = Array2d::init([-1, -1]..=[1, 1], |v: IVec2| v.x);
    arr.resize([-2, -2]..=[2, 2]);
    iter_eq(
        arr.rows(),
        [
            &[0, 0, 0, 0, 0],
            &[0, -1, 0, 1, 0],
            &[0, -1, 0, 1, 0],
            &[0, -1, 0, 1, 0],
            &[0, 0, 0, 0, 0],
        ] as [&[_]; 5],
    );

    let mut arr = Array2d::init([-1, -1]..=[1, 1], |v: IVec2| v.y);
    arr.resize([0, 0]..=[0, 1]);
    iter_eq(arr.rows(), [&[0], &[1]] as [&[_]; 2]);
}

#[test]
pub fn insert() {
    let mut a = Array2d::<u32>::default();

    a.insert([4, 5], 2);

    assert_eq!(a.width(), 1);
    assert_eq!(a.height(), 1);
    assert_eq!(a.len(), 1);
    assert_eq!(a.fetch([4, 5]), 2);

    a.insert([5, 7], 4);

    assert_eq!(a.width(), 2);
    assert_eq!(a.height(), 3);
    assert_eq!(a.len(), 6);
    assert_eq!(a.fetch([4, 5]), 2);
    assert_eq!(a.fetch([5, 7]), 4);
    assert_eq!(a.fetch([5, 3]), 0);

    a.insert([5, 3], 3);

    assert_eq!(a.width(), 2);
    assert_eq!(a.height(), 5);
    assert_eq!(a.len(), 10);
    assert_eq!(a.fetch([4, 5]), 2);
    assert_eq!(a.fetch([5, 7]), 4);
    assert_eq!(a.fetch([5, 3]), 3);

    a.insert([4, 4], 3);

    assert_eq!(a.width(), 2);
    assert_eq!(a.height(), 5);
    assert_eq!(a.len(), 10);
    assert_eq!(a.fetch([4, 5]), 2);
    assert_eq!(a.fetch([5, 7]), 4);
    assert_eq!(a.fetch([5, 3]), 3);
    assert_eq!(a.fetch([4, 4]), 3);
}

#[test]
pub fn paint() {
    let mut canvas = Array2d::<i32>::new([-2, -2]..=[2, 2]);
    let brush = Array2d::<i32>::init([-2, -2]..=[2, 2], |v: IVec2| {
        (3 - (v.x.abs() + v.y.abs())).max(0)
    });

    canvas.paint(&brush, [0, 0], |a, b| *a += *b);
    iter_eq(
        canvas.rows(),
        [
            &[0, 0, 1, 0, 0],
            &[0, 1, 2, 1, 0],
            &[1, 2, 3, 2, 1],
            &[0, 1, 2, 1, 0],
            &[0, 0, 1, 0, 0],
        ] as [&[_]; 5],
    );

    canvas.paint(&brush, [0, -2], |a, b| *a += *b);

    iter_eq(
        canvas.rows(),
        [
            &[1, 2, 4, 2, 1],
            &[0, 2, 4, 2, 0],
            &[1, 2, 4, 2, 1],
            &[0, 1, 2, 1, 0],
            &[0, 0, 1, 0, 0],
        ] as [&[_]; 5],
    );

    let mut canvas = Array2d::<i32>::new([-2, -2]..=[2, 2]);
    let brush = Array2d::<i32>::init([-2, -2]..=[2, 2], |v: IVec2| {
        (3 - (v.x.abs() + v.y.abs())).max(0)
    });

    canvas.paint(&brush, [2, 1], |a, b| *a += *b);
    iter_eq(
        canvas.rows(),
        [
            &[0, 0, 0, 0, 0],
            &[0, 0, 0, 0, 1],
            &[0, 0, 0, 1, 2],
            &[0, 0, 1, 2, 3],
            &[0, 0, 0, 1, 2],
        ] as [&[_]; 5],
    );
}

#[test]
pub fn expand() {
    let mut a = Array2d::from_vec(vec![1, 2, 3, 4], [2, 0]..=[3, 1]);
    let b = Array2d::from_vec(vec![1, 2, 3, 4], [0, 2]..=[1, 3]);

    a.merge(&b);

    iter_eq(
        a.rows(),
        [&[0, 0, 1, 2], &[0, 0, 3, 4], &[1, 2, 0, 0], &[3, 4, 0, 0]] as [&[_]; 4],
    )
}

#[test]
pub fn zip() {
    let mut a = Array2d::from_vec(vec![9, 4, 7, 3, 6, 1, 2, 8, 5], [0, 0]..=[2, 2]);
    let b = Array2d::from_vec(vec![5, 9, 4, 7, 3, 6, 1, 2, 8], [0, 0]..=[2, 2]);

    let v = a.zip(&b).map(|a, b| a > b);

    iter_eq(
        v.rows(),
        [
            &[true, false, true],
            &[false, true, false],
            &[true, true, false],
        ] as [&[_]; 3],
    );

    a.zip_mut(&b).for_each_mut(|a, b| *a += *b);

    iter_eq(
        a.rows(),
        [&[14, 13, 11], &[10, 9, 7], &[3, 10, 13]] as [&[_]; 3],
    );
}
