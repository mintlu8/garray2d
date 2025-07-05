use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{
    Boundary, GenericArray2d,
    serde::ser::SerializeRows,
    traits::{Array2dStorage, Array2dStorageOwned},
};

impl<T: Array2dStorage<Item: Serialize>> Serialize for GenericArray2d<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        ser::Array2d {
            min: self.boundary.min.into(),
            dimension: self.boundary.dimension.into(),
            data: SerializeRows(self.len(), || self.values()),
        }
        .serialize(serializer)
    }
}

impl<'de, T: Array2dStorageOwned<Item: Deserialize<'de>>> Deserialize<'de> for GenericArray2d<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let result = <de::Array2d<T::Item>>::deserialize(deserializer)?;
        let boundary = Boundary {
            min: result.min.into(),
            dimension: result.dimension.into(),
        };
        if result.data.len() < boundary.len() {
            return Err(serde::de::Error::custom(
                "Array2d does not contain enough items.",
            ));
        }
        Ok(GenericArray2d {
            data: T::from_vec(result.data),
            boundary,
            pitch: boundary.pitch(),
        })
    }
}

mod ser {
    use serde::{Serialize, Serializer, ser::SerializeSeq};

    pub(crate) struct SerializeRows<T>(pub usize, pub T);

    impl<T: Fn() -> I, I: IntoIterator<Item: Serialize>> Serialize for SerializeRows<T> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            let mut seq = serializer.serialize_seq(Some(self.0))?;
            for value in (self.1)().into_iter() {
                seq.serialize_element(&value)?;
            }
            seq.end()
        }
    }

    #[derive(Serialize)]
    #[serde(bound(serialize = "SerializeRows<T>: Serialize"))]
    pub(crate) struct Array2d<T> {
        pub min: [i32; 2],
        pub dimension: [u32; 2],
        pub data: SerializeRows<T>,
    }
}

impl Serialize for Boundary {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        boundary::Boundary {
            min: self.min.into(),
            dimension: self.dimension.into(),
        }
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Boundary {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let boundary = <boundary::Boundary>::deserialize(deserializer)?;
        Ok(Boundary {
            min: boundary.min.into(),
            dimension: boundary.dimension.into(),
        })
    }
}

mod de {
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub(crate) struct Array2d<T> {
        pub min: [i32; 2],
        pub dimension: [u32; 2],
        pub data: Vec<T>,
    }
}

mod boundary {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub(crate) struct Boundary {
        pub min: [i32; 2],
        pub dimension: [u32; 2],
    }
}
