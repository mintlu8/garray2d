#[cfg(feature = "serde")]
#[test]
pub fn serde() {
    use garray2d::Array2d;
    use glam::IVec2;
    use serde_json::json;

    let a = Array2d::<i32>::default();
    let b = serde_json::to_value(&a).unwrap();

    assert_eq!(
        b,
        json!({
            "min": [0, 0],
            "dimension": [0, 0],
            "data": [],
        })
    );

    let c: Array2d<i32> = serde_json::from_value(b).unwrap();
    assert_eq!(a, c);

    let a = Array2d::<i32>::init([1, 2]..=[4, 6], |v: IVec2| v.x + v.y);
    let b = serde_json::to_value(&a).unwrap();

    assert_eq!(
        b,
        json!({
            "min": [1, 2],
            "dimension": [4, 5],
            "data": [
                3, 4, 5, 6,
                4, 5, 6, 7,
                5, 6, 7, 8,
                6, 7, 8, 9,
                7, 8, 9, 10,
            ],
        })
    );

    let c: Array2d<i32> = serde_json::from_value(b).unwrap();
    assert_eq!(a, c);

    let d = a.slice(..);
    assert_eq!(a, d);

    let b = serde_json::to_value(d).unwrap();

    assert_eq!(
        b,
        json!({
            "min": [1, 2],
            "dimension": [4, 5],
            "data": [
                3, 4, 5, 6,
                4, 5, 6, 7,
                5, 6, 7, 8,
                6, 7, 8, 9,
                7, 8, 9, 10,
            ],
        })
    );

    let d = a.slice([3, 4]..);

    let b = serde_json::to_value(d).unwrap();

    assert_eq!(
        b,
        json!({
            "min": [3, 4],
            "dimension": [2, 3],
            "data": [
                7, 8,
                8, 9,
                9, 10,
            ],
        })
    );
}
